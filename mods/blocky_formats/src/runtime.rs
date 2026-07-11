use crate::{BlockyAnimation, BlockyModel, BlockyNode, Vec3f};
use std::collections::HashMap;

pub type NodeIndex = usize;

/// A flattened, game-friendly model hierarchy.
#[derive(Debug, Clone)]
pub struct RuntimeModel {
    pub nodes: Vec<RuntimeNode>,
    pub roots: Vec<NodeIndex>,
    pub nodes_by_id: HashMap<String, NodeIndex>,
    pub nodes_by_name: HashMap<String, Vec<NodeIndex>>,
}

impl RuntimeModel {
    pub fn from_model(model: &BlockyModel) -> Self {
        let mut runtime = Self {
            nodes: Vec::new(),
            roots: Vec::new(),
            nodes_by_id: HashMap::new(),
            nodes_by_name: HashMap::new(),
        };

        for node in &model.nodes {
            let index = runtime.push_node_recursive(node, None);
            runtime.roots.push(index);
        }

        runtime
    }

    pub fn node(&self, index: NodeIndex) -> Option<&RuntimeNode> {
        self.nodes.get(index)
    }

    pub fn node_by_id(&self, id: &str) -> Option<&RuntimeNode> {
        self.nodes_by_id.get(id).and_then(|&idx| self.nodes.get(idx))
    }

    /// Returns the node pivot relative to the parent pivot.
    ///
    /// In `.blockymodel`, a child node's encoded `position` is relative to the
    /// center of its parent's main shape, not directly to the parent's pivot.
    /// The parent's `shape.offset` therefore has to be restored when building
    /// a transform hierarchy. Root nodes have no parent offset.
    pub fn resolved_local_position(&self, index: NodeIndex) -> Option<Vec3f> {
        let node = self.nodes.get(index)?;
        let parent_offset = node
            .parent
            .and_then(|parent| self.nodes.get(parent))
            .and_then(|parent| parent.shape.as_ref())
            .map(|shape| shape.offset)
            .unwrap_or(Vec3f::ZERO);

        Some(Vec3f::new(
            node.position.x + parent_offset.x,
            node.position.y + parent_offset.y,
            node.position.z + parent_offset.z,
        ))
    }

    /// `.blockyanim` addresses nodes by name. This returns all matching nodes.
    pub fn node_indices_by_name(&self, name: &str) -> &[NodeIndex] {
        self.nodes_by_name
            .get(name)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }


    /// Checks name-based compatibility between this model and an animation.
    ///
    /// `.blockyanim` targets nodes by name. Extra animation tracks are harmless but will not affect
    /// this model. Model nodes without animation tracks keep their rest transform.
    pub fn check_animation_compatibility(&self, animation: &BlockyAnimation) -> AnimationCompatibility {
        let mut matched_node_names = Vec::new();
        let mut animation_nodes_missing_in_model = Vec::new();
        let mut model_nodes_without_animation = Vec::new();

        for name in animation.node_animations.keys() {
            if self.nodes_by_name.contains_key(name) {
                matched_node_names.push(name.clone());
            } else {
                animation_nodes_missing_in_model.push(name.clone());
            }
        }

        for name in self.nodes_by_name.keys() {
            if !animation.node_animations.contains_key(name) {
                model_nodes_without_animation.push(name.clone());
            }
        }

        AnimationCompatibility {
            matched_node_names,
            animation_nodes_missing_in_model,
            model_nodes_without_animation,
        }
    }

    fn push_node_recursive(&mut self, node: &BlockyNode, parent: Option<NodeIndex>) -> NodeIndex {
        let index = self.nodes.len();

        self.nodes_by_id.insert(node.id.clone(), index);
        self.nodes_by_name
            .entry(node.name.clone())
            .or_default()
            .push(index);

        self.nodes.push(RuntimeNode {
            id: node.id.clone(),
            name: node.name.clone(),
            parent,
            children: Vec::new(),
            position: node.position,
            orientation: node.orientation,
            shape: node.shape.clone(),
        });

        let child_indices: Vec<NodeIndex> = node
            .children
            .iter()
            .map(|child| self.push_node_recursive(child, Some(index)))
            .collect();

        self.nodes[index].children = child_indices;
        index
    }
}

impl From<&BlockyModel> for RuntimeModel {
    fn from(model: &BlockyModel) -> Self {
        Self::from_model(model)
    }
}

/// A single flattened node.
#[derive(Debug, Clone)]
pub struct RuntimeNode {
    pub id: String,
    pub name: String,
    pub parent: Option<NodeIndex>,
    pub children: Vec<NodeIndex>,
    pub position: crate::Vec3f,
    pub orientation: crate::Quatf,
    pub shape: Option<crate::BlockyShape>,
}


/// Name-based compatibility report between a `RuntimeModel` and a `BlockyAnimation`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnimationCompatibility {
    pub matched_node_names: Vec<String>,
    pub animation_nodes_missing_in_model: Vec<String>,
    pub model_nodes_without_animation: Vec<String>,
}

impl AnimationCompatibility {
    pub fn is_fully_matched(&self) -> bool {
        self.animation_nodes_missing_in_model.is_empty() && self.model_nodes_without_animation.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolved_local_position_restores_parent_shape_offset() {
        let model = BlockyModel::from_str(
            r#"{
                "nodes": [{
                    "id": "root",
                    "name": "Root",
                    "position": { "x": 0, "y": 51, "z": 0 },
                    "shape": {
                        "type": "box",
                        "offset": { "x": 2, "y": 8, "z": -1 },
                        "stretch": { "x": 1, "y": 1, "z": 1 },
                        "settings": { "size": { "x": 26, "y": 16, "z": 18 } }
                    },
                    "children": [{
                        "id": "child",
                        "name": "Child",
                        "position": { "x": 1, "y": 5, "z": -3 }
                    }]
                }]
            }"#,
        )
        .expect("test model should parse");
        let runtime = RuntimeModel::from(&model);

        assert_eq!(
            runtime.resolved_local_position(runtime.roots[0]),
            Some(Vec3f::new(0.0, 51.0, 0.0))
        );
        let child = runtime.nodes_by_id["child"];
        assert_eq!(
            runtime.resolved_local_position(child),
            Some(Vec3f::new(3.0, 13.0, -4.0))
        );
    }
}
