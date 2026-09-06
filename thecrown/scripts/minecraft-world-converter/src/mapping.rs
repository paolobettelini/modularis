use serde::Serialize;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
    path::Path,
};

const DEFAULT_SHAPE_SUFFIXES: &[&str] = &[
    "_wall_hanging_sign",
    "_hanging_sign",
    "_wall_sign",
    "_fence_gate",
    "_pressure_plate",
    "_trapdoor",
    "_stairs",
    "_slab",
    "_wall",
    "_fence",
    "_button",
    "_door",
    "_carpet",
    "_bed",
    "_banner",
    "_pane",
    "_sign",
];

const COLORS: &[&str] = &[
    "light_blue",
    "light_gray",
    "white",
    "orange",
    "magenta",
    "yellow",
    "lime",
    "pink",
    "gray",
    "cyan",
    "purple",
    "blue",
    "brown",
    "green",
    "red",
    "black",
];

const WOOD_FAMILIES: &[&str] = &[
    "dark_oak",
    "pale_oak",
    "oak",
    "spruce",
    "birch",
    "jungle",
    "acacia",
    "cherry",
    "mangrove",
    "bamboo",
    "crimson",
    "warped",
];

const IGNORED_TOKENS: &[&str] = &[
    "block",
    "stairs",
    "slab",
    "wall",
    "fence",
    "gate",
    "door",
    "trapdoor",
    "button",
    "pressure",
    "plate",
    "sign",
    "hanging",
    "carpet",
    "bed",
    "banner",
    "pane",
];

#[derive(Debug, Clone)]
struct Rule {
    kind: RuleKind,
    needle: String,
    target: String,
}

#[derive(Debug, Clone, Copy)]
enum RuleKind {
    Prefix,
    Suffix,
    Contains,
}

#[derive(Debug, Clone)]
pub struct MappingConfig {
    fallback: String,
    fuzzy: bool,
    minimum_fuzzy_score: i32,
    shape_suffixes: Vec<String>,
    exact: HashMap<String, String>,
    materials: HashMap<String, String>,
    rules: Vec<Rule>,
}

impl MappingConfig {
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let text = fs::read_to_string(path)?;
        let mut config = Self {
            fallback: "demo:stone".to_owned(),
            fuzzy: true,
            minimum_fuzzy_score: 7,
            shape_suffixes: DEFAULT_SHAPE_SUFFIXES
                .iter()
                .map(|value| (*value).to_owned())
                .collect(),
            exact: HashMap::new(),
            materials: HashMap::new(),
            rules: Vec::new(),
        };

        let mut section = String::new();
        for (line_number, raw_line) in text.lines().enumerate() {
            let line = strip_comment(raw_line).trim();
            if line.is_empty() {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                section = line[1..line.len() - 1].trim().to_owned();
                continue;
            }

            let Some((raw_key, raw_value)) = line.split_once('=') else {
                return Err(format!(
                    "invalid mapping line {} in '{}': expected key = value",
                    line_number + 1,
                    path.display()
                )
                .into());
            };

            let key = parse_scalar(raw_key.trim());
            let value = parse_scalar(raw_value.trim());

            match section.as_str() {
                "settings" => match key.as_str() {
                    "fallback" => config.fallback = value,
                    "fuzzy" => config.fuzzy = parse_bool(&value, path, line_number + 1)?,
                    "minimum_fuzzy_score" => {
                        config.minimum_fuzzy_score = value.parse().map_err(|_| {
                            format!(
                                "invalid integer at {}:{}",
                                path.display(),
                                line_number + 1
                            )
                        })?;
                    }
                    "shape_suffixes" => {
                        config.shape_suffixes = value
                            .split(',')
                            .map(str::trim)
                            .filter(|value| !value.is_empty())
                            .map(ToOwned::to_owned)
                            .collect();
                    }
                    other => {
                        return Err(format!(
                            "unknown setting '{other}' at {}:{}",
                            path.display(),
                            line_number + 1
                        )
                        .into());
                    }
                },
                "exact" => {
                    config.exact.insert(key, value);
                }
                "materials" => {
                    config.materials.insert(normalize_path(&key), value);
                }
                "rules" => {
                    let Some((kind, needle)) = key.split_once(':') else {
                        return Err(format!(
                            "invalid rule '{}' at {}:{}; expected prefix:, suffix: or contains:",
                            key,
                            path.display(),
                            line_number + 1
                        )
                        .into());
                    };
                    let kind = match kind {
                        "prefix" => RuleKind::Prefix,
                        "suffix" => RuleKind::Suffix,
                        "contains" => RuleKind::Contains,
                        _ => {
                            return Err(format!(
                                "unknown rule kind '{kind}' at {}:{}",
                                path.display(),
                                line_number + 1
                            )
                            .into());
                        }
                    };
                    config.rules.push(Rule {
                        kind,
                        needle: normalize_path(needle),
                        target: value,
                    });
                }
                "" => {
                    return Err(format!(
                        "mapping entry outside a section at {}:{}",
                        path.display(),
                        line_number + 1
                    )
                    .into());
                }
                other => {
                    return Err(format!(
                        "unknown mapping section '[{other}]' in '{}'",
                        path.display()
                    )
                    .into());
                }
            }
        }

        if config.shape_suffixes.is_empty() {
            config.shape_suffixes = DEFAULT_SHAPE_SUFFIXES
                .iter()
                .map(|value| (*value).to_owned())
                .collect();
        }
        config
            .shape_suffixes
            .sort_by_key(|suffix| std::cmp::Reverse(suffix.len()));

        Ok(config)
    }

    pub fn disable_fuzzy(&mut self) {
        self.fuzzy = false;
    }

    fn referenced_targets(&self) -> impl Iterator<Item = &str> {
        std::iter::once(self.fallback.as_str())
            .chain(self.exact.values().map(String::as_str))
            .chain(self.materials.values().map(String::as_str))
            .chain(self.rules.iter().map(|rule| rule.target.as_str()))
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MappingDecision {
    #[serde(skip)]
    pub block: u32,
    pub target: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
}

#[derive(Debug, Clone)]
struct TargetDescriptor {
    id: String,
    path: String,
    tokens: Vec<String>,
    color: Option<&'static str>,
    wood: Option<&'static str>,
}

pub struct BlockMapper {
    blocks: HashMap<String, u32>,
    config: MappingConfig,
    targets: Vec<TargetDescriptor>,
    fallback: u32,
}

impl BlockMapper {
    pub fn new(
        blocks: HashMap<String, u32>,
        config: MappingConfig,
    ) -> Result<Self, Box<dyn Error>> {
        let missing = config
            .referenced_targets()
            .filter(|id| !blocks.contains_key(*id))
            .collect::<HashSet<_>>();
        if !missing.is_empty() {
            let mut missing = missing.into_iter().collect::<Vec<_>>();
            missing.sort_unstable();
            return Err(format!(
                "mapping file references target blocks that are not available in --mods: {}",
                missing.join(", ")
            )
            .into());
        }

        let fallback = blocks[&config.fallback];
        let mut targets = blocks
            .keys()
            .map(|id| {
                let path = id
                    .split_once(':')
                    .map(|(_, path)| normalize_path(path))
                    .unwrap_or_else(|| normalize_path(id));
                TargetDescriptor {
                    id: id.clone(),
                    tokens: tokens(&path),
                    color: find_tag(&path, COLORS),
                    wood: find_tag(&path, WOOD_FAMILIES),
                    path,
                }
            })
            .collect::<Vec<_>>();
        targets.sort_by(|left, right| left.id.cmp(&right.id));

        Ok(Self {
            blocks,
            config,
            targets,
            fallback,
        })
    }

    pub fn map(&self, source: &str) -> MappingDecision {
        if let Some(target) = self.config.exact.get(source) {
            return self.decision(target, "explicit", None);
        }

        let Some((namespace, raw_path)) = source.split_once(':') else {
            return self.fallback("fallback");
        };
        if namespace != "minecraft" {
            return self.fallback("fallback-non-minecraft");
        }

        let path = normalize_path(raw_path);

        if let Some(target) = self.direct_target(&path) {
            return self.decision(&target, "exact-name", None);
        }

        for candidate in self.variant_candidates(&path) {
            if let Some(target) = self.direct_target(&candidate) {
                return self.decision(&target, "normalized-variant", None);
            }
            if let Some(target) = self.config.materials.get(&candidate) {
                return self.decision(target, "material", None);
            }
        }

        if let Some(target) = self.config.materials.get(&path) {
            return self.decision(target, "material", None);
        }

        for rule in &self.config.rules {
            let matches = match rule.kind {
                RuleKind::Prefix => path.starts_with(&rule.needle),
                RuleKind::Suffix => path.ends_with(&rule.needle),
                RuleKind::Contains => path.contains(&rule.needle),
            };
            if matches {
                return self.decision(&rule.target, "rule", None);
            }
        }

        if self.config.fuzzy {
            if let Some((target, score)) = self.closest_target(&path) {
                if score >= self.config.minimum_fuzzy_score {
                    return self.decision(&target, "semantic", Some(score));
                }
            }
        }

        self.fallback("fallback")
    }

    fn direct_target(&self, path: &str) -> Option<String> {
        let target = format!("demo:{}", path.replace('_', "-"));
        self.blocks.contains_key(&target).then_some(target)
    }

    fn decision(&self, target: &str, method: &str, score: Option<i32>) -> MappingDecision {
        MappingDecision {
            block: self.blocks[target],
            target: target.to_owned(),
            method: method.to_owned(),
            score,
        }
    }

    fn fallback(&self, method: &str) -> MappingDecision {
        MappingDecision {
            block: self.fallback,
            target: self.config.fallback.clone(),
            method: method.to_owned(),
            score: None,
        }
    }

    fn variant_candidates(&self, path: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut queue = vec![path.to_owned()];
        let mut seen = HashSet::new();

        while let Some(candidate) = queue.pop() {
            if !seen.insert(candidate.clone()) {
                continue;
            }
            result.push(candidate.clone());

            if let Some(stripped) = candidate.strip_prefix("waxed_") {
                queue.push(stripped.to_owned());
            }
            if let Some(stripped) = candidate.strip_prefix("infested_") {
                queue.push(stripped.to_owned());
            }

            for (from, to) in [
                ("_wood", "_log"),
                ("_hyphae", "_stem"),
                ("_brick", "_bricks"),
                ("_tile", "_tiles"),
            ] {
                if let Some(base) = candidate.strip_suffix(from) {
                    queue.push(format!("{base}{to}"));
                }
            }

            for suffix in &self.config.shape_suffixes {
                if let Some(base) = candidate.strip_suffix(suffix) {
                    queue.push(base.to_owned());
                }
            }
        }

        result
    }

    fn closest_target(&self, source: &str) -> Option<(String, i32)> {
        let source_tokens = tokens(source);
        let source_color = find_tag(source, COLORS);
        let source_wood = find_tag(source, WOOD_FAMILIES);

        self.targets
            .iter()
            .filter_map(|target| {
                let score = semantic_score(
                    source,
                    &source_tokens,
                    source_color,
                    source_wood,
                    target,
                );
                (score > 0).then_some((target.id.clone(), score))
            })
            .max_by(|left, right| {
                left.1
                    .cmp(&right.1)
                    .then_with(|| right.0.cmp(&left.0))
            })
    }
}

fn semantic_score(
    source: &str,
    source_tokens: &[String],
    source_color: Option<&str>,
    source_wood: Option<&str>,
    target: &TargetDescriptor,
) -> i32 {
    let mut score = 0;

    for token in source_tokens {
        if IGNORED_TOKENS.contains(&token.as_str()) {
            continue;
        }
        if target.tokens.contains(token) {
            score += token_weight(token);
        }
    }

    match (source_color, target.color) {
        (Some(left), Some(right)) if left == right => score += 7,
        (Some(_), Some(_)) => score -= 12,
        _ => {}
    }

    match (source_wood, target.wood) {
        (Some(left), Some(right)) if left == right => score += 8,
        (Some(_), Some(_)) => score -= 14,
        _ => {}
    }

    let preferences = [
        ((contains_any(source, &["_ore", "ore_"]) || source.ends_with("ore")), "ore", 6),
        ((source.contains("glass")), "glass", 6),
        ((contains_any(source, &["wool", "carpet", "_bed", "banner"])), "wool", 6),
        ((source.contains("concrete")), "concrete", 5),
        ((source.contains("terracotta")), "terracotta", 5),
        ((source.contains("leaves")), "leaves", 7),
        ((contains_any(source, &["_log", "_wood"])), "log", 6),
        ((contains_any(source, &["_stem", "hyphae"])), "stem", 6),
        ((source.contains("coral")), "coral", 5),
        ((source.contains("copper")), "copper", 4),
        ((source.contains("sandstone")), "sandstone", 5),
        ((source.contains("deepslate")), "deepslate", 5),
        ((source.contains("blackstone")), "blackstone", 5),
        ((source.contains("prismarine")), "prismarine", 5),
        ((source.contains("quartz")), "quartz", 5),
    ];

    for (applies, token, bonus) in preferences {
        if applies && target.path.contains(token) {
            score += bonus;
        }
    }

    if source_wood.is_some()
        && self_is_shape_variant(source)
        && target.path.contains("planks")
    {
        score += 6;
    }

    score
}

fn self_is_shape_variant(path: &str) -> bool {
    DEFAULT_SHAPE_SUFFIXES.iter().any(|suffix| path.ends_with(suffix))
}

fn token_weight(token: &str) -> i32 {
    if COLORS.contains(&token) {
        7
    } else if [
        "stone",
        "deepslate",
        "blackstone",
        "sandstone",
        "quartz",
        "prismarine",
        "copper",
        "iron",
        "gold",
        "diamond",
        "emerald",
        "netherite",
        "redstone",
        "lapis",
        "coal",
        "amethyst",
        "tuff",
        "terracotta",
        "concrete",
        "glass",
        "wool",
        "coral",
    ]
    .contains(&token)
    {
        6
    } else if [
        "polished",
        "smooth",
        "chiseled",
        "cracked",
        "cut",
        "exposed",
        "weathered",
        "oxidized",
        "stripped",
        "mossy",
    ]
    .contains(&token)
    {
        4
    } else {
        3
    }
}

fn tokens(path: &str) -> Vec<String> {
    path.split('_')
        .filter(|token| !token.is_empty())
        .map(|token| match token {
            "bricks" => "brick",
            "planks" => "plank",
            "leaves" => "leaf",
            "tiles" => "tile",
            "ores" => "ore",
            other => other,
        })
        .map(ToOwned::to_owned)
        .collect()
}

fn find_tag(path: &str, candidates: &'static [&'static str]) -> Option<&'static str> {
    candidates
        .iter()
        .copied()
        .find(|candidate| has_component_sequence(path, candidate))
}

fn has_component_sequence(path: &str, needle: &str) -> bool {
    path == needle
        || path.starts_with(&format!("{needle}_"))
        || path.ends_with(&format!("_{needle}"))
        || path.contains(&format!("_{needle}_"))
}

fn contains_any(value: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| value.contains(needle))
}

fn normalize_path(value: &str) -> String {
    value.trim().replace('-', "_")
}

fn strip_comment(line: &str) -> &str {
    let mut in_string = false;
    let mut escaped = false;
    for (index, character) in line.char_indices() {
        match character {
            '\\' if in_string => escaped = !escaped,
            '"' if !escaped => in_string = !in_string,
            '#' if !in_string => return &line[..index],
            _ => escaped = false,
        }
    }
    line
}

fn parse_scalar(value: &str) -> String {
    let value = value.trim();
    if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
        value[1..value.len() - 1]
            .replace("\\\"", "\"")
            .replace("\\\\", "\\")
    } else {
        value.to_owned()
    }
}

fn parse_bool(value: &str, path: &Path, line: usize) -> Result<bool, Box<dyn Error>> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(format!("invalid boolean at {}:{line}", path.display()).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mapper() -> BlockMapper {
        let ids = [
            "demo:air",
            "demo:stone",
            "demo:oak-planks",
            "demo:spruce-planks",
            "demo:stone-bricks",
            "demo:blue-stained-glass",
            "demo:orange-stained-glass",
            "demo:short-grass",
            "demo:iron-block",
        ];
        let blocks = ids
            .iter()
            .enumerate()
            .map(|(index, id)| ((*id).to_owned(), index as u32))
            .collect();
        let config = MappingConfig {
            fallback: "demo:stone".to_owned(),
            fuzzy: true,
            minimum_fuzzy_score: 7,
            shape_suffixes: DEFAULT_SHAPE_SUFFIXES
                .iter()
                .map(|value| (*value).to_owned())
                .collect(),
            exact: HashMap::from([
                ("minecraft:water".to_owned(), "demo:blue-stained-glass".to_owned()),
                ("minecraft:lava".to_owned(), "demo:orange-stained-glass".to_owned()),
            ]),
            materials: HashMap::from([
                ("oak".to_owned(), "demo:oak-planks".to_owned()),
                ("spruce".to_owned(), "demo:spruce-planks".to_owned()),
                ("stone_brick".to_owned(), "demo:stone-bricks".to_owned()),
            ]),
            rules: vec![Rule {
                kind: RuleKind::Suffix,
                needle: "_sapling".to_owned(),
                target: "demo:short-grass".to_owned(),
            }],
        };
        BlockMapper::new(blocks, config).unwrap()
    }

    #[test]
    fn maps_exact_override() {
        assert_eq!(mapper().map("minecraft:water").target, "demo:blue-stained-glass");
    }

    #[test]
    fn maps_wood_shape_to_planks() {
        assert_eq!(mapper().map("minecraft:spruce_stairs").target, "demo:spruce-planks");
    }

    #[test]
    fn maps_material_alias() {
        assert_eq!(mapper().map("minecraft:stone_brick_wall").target, "demo:stone-bricks");
    }

    #[test]
    fn maps_rule() {
        assert_eq!(mapper().map("minecraft:birch_sapling").target, "demo:short-grass");
    }
}
