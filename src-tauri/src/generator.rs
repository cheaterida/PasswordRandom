use rand::Rng;
use regex::Regex;

const WORDLIST: &[&str] = &[
    "abandon", "ability", "able", "about", "above", "absent", "absorb", "abstract", "abuse", "access",
    "accident", "account", "achieve", "acid", "across", "action", "active", "actor", "actual", "adapt",
    "add", "address", "adjust", "admit", "adopt", "adult", "advance", "advice", "affair", "affect",
    "afford", "afraid", "after", "again", "age", "agent", "agree", "ahead", "aid", "aim",
    "air", "airport", "alarm", "album", "alert", "alien", "align", "alive", "allow", "almost",
    "alone", "along", "alpha", "already", "also", "alter", "always", "amaze", "amount", "amuse",
    "anchor", "angel", "anger", "angle", "angry", "animal", "ankle", "annual", "answer", "antique",
    "anxiety", "apart", "apple", "apply", "area", "argue", "arm", "army", "around", "arrange",
    "arrest", "arrive", "arrow", "art", "artist", "aside", "ask", "aspect", "assault", "asset",
    "assist", "assume", "assure", "athlete", "attach", "attack", "attempt", "attend", "attract", "author",
    "auto", "autumn", "avail", "avenue", "average", "avoid", "awake", "award", "aware", "baby",
    "back", "bad", "badly", "bag", "bake", "balance", "ball", "ban", "band", "bank",
    "bar", "barely", "barrel", "base", "basic", "basis", "basket", "battle", "bay", "beach",
    "bean", "bear", "beat", "beauty", "become", "bedroom", "beer", "before", "begin", "behave",
    "behind", "believe", "below", "belt", "bench", "bend", "benefit", "best", "beyond", "bike",
    "bill", "bind", "bird", "birth", "bite", "bitter", "black", "blade", "blame", "blank",
    "blast", "bleed", "blend", "bless", "blind", "block", "blood", "blow", "blue", "board",
    "boat", "body", "bomb", "bond", "bone", "bonus", "book", "boost", "border", "borrow",
    "boss", "both", "bother", "bottle", "bottom", "bounce", "bowl", "box", "boy", "brain",
    "branch", "brand", "brave", "bread", "break", "breast", "breath", "brick", "bridge", "brief",
    "bright", "bring", "broad", "broken", "brother", "brown", "brush", "bubble", "budget", "build",
    "bullet", "bunch", "burden", "burn", "burst", "bus", "busy", "but", "butter", "button",
    "buy", "buyer", "cabin", "cable", "cage", "cake", "call", "calm", "camera", "camp",
    "can", "cancel", "candle", "candy", "cap", "capital", "captain", "capture", "carbon", "card",
    "care", "career", "carry", "case", "cash", "cast", "castle", "cat", "catch", "category",
    "cattle", "cause", "ceiling", "cell", "center", "chain", "chair", "chalk", "chance", "change",
    "chaos", "chapter", "charge", "charm", "chart", "chase", "cheap", "check", "cheese", "chef",
    "cherry", "chest", "chicken", "chief", "child", "china", "chip", "choice", "choose", "church",
    "circle", "cite", "citizen", "city", "civil", "claim", "class", "classic", "clay", "clean",
    "clear", "clerk", "click", "client", "climb", "clock", "close", "closet", "cloud", "club",
    "clue", "coach", "coal", "coast", "coat", "code", "coffee", "coin", "cold", "collapse",
    "college", "color", "column", "combine", "come", "comedy", "comfort", "command", "comment", "commit",
    "common", "company", "compare", "compete", "complex", "concern", "conduct", "confirm", "connect", "consist",
    "contact", "contain", "content", "contest", "context", "control", "convert", "cook", "cool", "cope",
    "copy", "core", "corner", "correct", "cost", "cotton", "couch", "could", "council", "count",
    "country", "county", "couple", "course", "court", "cousin", "cover", "crack", "craft", "crash",
    "crazy", "cream", "create", "credit", "crew", "crime", "crisis", "crop", "cross", "crowd",
    "crucial", "cry", "culture", "cup", "curious", "current", "curve", "custom", "cut", "cycle",
    "dad", "damage", "dance", "danger", "dare", "dark", "data", "date", "daughter", "day",
    "dead", "deal", "dear", "death", "debate", "debt", "decade", "decide", "decision", "deck",
    "declare", "decline", "deep", "defeat", "defend", "define", "degree", "delay", "deliver", "demand",
    "deny", "depart", "depend", "deposit", "depth", "derive", "design", "desire", "desk", "detail",
    "detect", "develop", "device", "devote", "dialog", "diamond", "diet", "differ", "digital", "dinner",
    "direct", "dirty", "disc", "dislike", "display", "distance", "dive", "divide", "divorce", "doctor",
    "document", "dog", "doll", "dollar", "domain", "donate", "donor", "door", "dose", "double",
    "doubt", "down", "draft", "dragon", "drama", "draw", "dream", "dress", "drink", "drive",
    "drop", "drug", "drum", "dry", "due", "dump", "during", "dust", "duty", "each",
    "eager", "ear", "early", "earn", "earth", "ease", "east", "easy", "eat", "echo",
    "edge", "edit", "educate", "effect", "effort", "egg", "eight", "either", "elbow", "elder",
    "elect", "element", "eleven", "else", "emerge", "emotion", "employ", "empty", "enable", "end",
    "enemy", "energy", "engage", "engine", "enhance", "enjoy", "enough", "ensure", "enter", "entire",
    "entry", "envelope", "equal", "escape", "essay", "estate", "even", "evening", "event", "ever",
    "every", "evidence", "evil", "exact", "example", "exchange", "excite", "excuse", "exist", "expand",
    "expect", "expert", "explain", "export", "expose", "extend", "extra", "eye", "fabric", "face",
    "fact", "factor", "fail", "faint", "fair", "faith", "fall", "false", "family", "famous",
    "fan", "fancy", "fantasy", "far", "farm", "fashion", "fast", "fat", "fate", "father",
    "fault", "favor", "fear", "feast", "feature", "federal", "feed", "feel", "female", "fence",
    "fiber", "fiction", "field", "fifteen", "fight", "figure", "file", "fill", "film", "final",
    "finance", "find", "fine", "finger", "finish", "fire", "firm", "first", "fish", "fix",
    "flag", "flame", "flash", "flat", "flavor", "flesh", "flight", "float", "flood", "floor",
    "flour", "flow", "flower", "fly", "focus", "fold", "folk", "follow", "food", "foot",
    "force", "forest", "forever", "forget", "fork", "form", "formal", "former", "forth", "fortune",
    "forum", "forward", "found", "frame", "free", "freedom", "freeze", "fresh", "friend", "front",
    "frost", "frozen", "fruit", "fuel", "full", "fun", "funny", "future", "gain", "galaxy",
    "gallery", "game", "gap", "garage", "garden", "garlic", "gas", "gate", "gather", "gaze",
    "gear", "gene", "general", "genius", "gentle", "genuine", "gesture", "get", "ghost", "giant",
    "gift", "girl", "give", "glad", "glance", "glass", "global", "glory", "glove", "glue",
    "goal", "god", "gold", "good", "govern", "grab", "grace", "grade", "grain", "grand",
    "grant", "grass", "grave", "great", "green", "ground", "group", "grow", "growth", "guard",
    "guess", "guest", "guide", "guilty", "guitar", "gun", "guy", "gym", "habit", "hair",
    "half", "hall", "hand", "handle", "hang", "happen", "happy", "hard", "harm", "harsh",
    "hat", "hate", "have", "head", "health", "hear", "heart", "heat", "heaven", "heavy",
    "height", "hello", "help", "hen", "here", "heritage", "hero", "hide", "high", "highlight",
    "highway", "hill", "hint", "hip", "hire", "history", "hit", "hobby", "hockey", "hold",
    "hole", "holiday", "home", "honest", "honey", "honor", "hook", "hope", "horizon", "horn",
    "horror", "horse", "host", "hotel", "hour", "house", "huge", "human", "humor", "hundred",
    "hungry", "hunt", "hurry", "hurt", "husband", "ice", "idea", "identify", "ignore", "image",
    "imagine", "impact", "import", "impose", "impress", "improve", "include", "income", "increase", "indeed",
    "index", "indicate", "industry", "infant", "inform", "initial", "injure", "inner", "input", "inquiry",
    "insect", "insert", "inside", "insist", "install", "intact", "interest", "into", "invest", "invite",
    "iron", "island", "isolate", "issue", "item", "itself", "jacket", "jail", "jam", "jet",
    "jewel", "job", "join", "joke", "journal", "journey", "joy", "judge", "juice", "jump",
    "junior", "jury", "just", "justice", "keen", "keep", "key", "kick", "kid", "kill",
    "kind", "king", "kiss", "kitchen", "knee", "knife", "knock", "know", "label", "labor",
    "lack", "ladder", "lady", "lake", "lamp", "land", "lane", "large", "laser", "last",
    "later", "latter", "laugh", "launch", "law", "lawyer", "layer", "lazy", "lead", "leaf",
    "league", "lean", "learn", "leave", "left", "leg", "legal", "lemon", "lend", "length",
    "lens", "lesson", "let", "letter", "level", "life", "lift", "light", "like", "likely",
    "limit", "line", "link", "lion", "list", "listen", "little", "live", "load", "loan",
    "local", "locate", "lock", "log", "logic", "lonely", "long", "loop", "lose", "loss",
    "lot", "loud", "love", "low", "loyal", "luck", "lucky", "lunch", "lung", "luxury",
    "machine", "mad", "magic", "main", "maintain", "major", "make", "male", "manage", "manner",
    "manual", "many", "map", "march", "margin", "mark", "market", "marry", "mask", "mass",
    "master", "match", "mate", "matter", "mature", "maximum", "maybe", "meal", "mean", "measure",
    "meat", "media", "medium", "meet", "member", "memory", "mental", "mention", "menu", "mercy",
    "mere", "merit", "mess", "metal", "method", "middle", "might", "mild", "military", "milk",
    "million", "mind", "mine", "minor", "minute", "miracle", "mirror", "miss", "mission", "mistake",
    "mix", "model", "modern", "modest", "mom", "moment", "money", "monitor", "month", "mood",
    "moon", "moral", "more", "morning", "most", "mother", "motion", "motor", "mountain", "mouse",
    "mouth", "move", "movie", "much", "murder", "muscle", "museum", "music", "must", "mutual",
    "mystery", "myth", "nail", "name", "narrow", "nasty", "nation", "nature", "navy", "near",
    "nearly", "neat", "need", "needle", "nerve", "network", "never", "new", "news", "next",
    "nice", "night", "nine", "no", "noble", "nobody", "nod", "noise", "none", "normal",
    "north", "nose", "note", "nothing", "notice", "notion", "novel", "now", "number", "nurse",
    "nut", "object", "obtain", "obvious", "occur", "ocean", "odd", "offense", "offer", "office",
    "often", "oil", "old", "once", "one", "online", "only", "onto", "open", "operate",
    "opinion", "option", "orange", "orbit", "order", "organ", "origin", "other", "ought", "outcome",
    "output", "outside", "over", "owe", "owner", "oxygen", "pace", "pack", "package", "page",
    "pain", "paint", "pair", "palace", "palm", "pan", "panel", "panic", "paper", "parent",
    "park", "part", "partner", "party", "pass", "passage", "passion", "past", "path", "patience",
    "pattern", "pause", "pay", "peace", "peak", "peer", "penalty", "people", "pepper", "perfect",
    "perform", "period", "permit", "person", "pet", "phone", "photo", "phrase", "physical", "piano",
    "pick", "piece", "pile", "pilot", "pin", "pine", "pink", "pipe", "pitch", "pizza",
    "place", "plain", "plan", "plane", "planet", "plant", "plate", "play", "please", "plenty",
    "plot", "plus", "pocket", "poem", "poet", "point", "police", "policy", "polish", "pool",
    "poor", "popular", "port", "portion", "pose", "position", "positive", "possible", "post", "pot",
    "potato", "power", "practice", "pray", "predict", "prefer", "prepare", "present", "press", "pretend",
    "pretty", "prevent", "price", "pride", "priest", "primary", "prince", "principle", "print", "prior",
    "prison", "private", "prize", "problem", "process", "produce", "profile", "program", "progress", "project",
    "promise", "prompt", "proof", "proper", "protect", "protein", "protest", "proud", "prove", "provide",
    "public", "pull", "punch", "pupil", "purchase", "pure", "purple", "purpose", "push", "put",
    "puzzle", "quality", "quarter", "queen", "question", "quick", "quiet", "quit", "quite", "quote",
    "race", "radical", "rain", "raise", "range", "rank", "rapid", "rare", "rate", "rather",
    "ratio", "raw", "reach", "react", "read", "ready", "real", "reality", "realize", "really",
    "reason", "rebel", "recall", "recent", "recipe", "record", "reduce", "refer", "reflect", "reform",
    "region", "register", "relate", "release", "relief", "remain", "remark", "remote", "remove", "rent",
    "repair", "repeat", "replace", "reply", "report", "request", "require", "reserve", "resolve", "resource",
    "respond", "rest", "result", "retain", "retire", "return", "reveal", "review", "revise", "reward",
    "rhythm", "rice", "rich", "ride", "rifle", "right", "ring", "riot", "rise", "risk",
    "river", "road", "robot", "rock", "rocket", "role", "roll", "roof", "room", "root",
    "rope", "rough", "round", "route", "row", "royal", "rule", "rumor", "run", "rural",
    "rush", "sacred", "sad", "safe", "sail", "salad", "salary", "sale", "salt", "same",
    "sample", "sand", "satellite", "sauce", "save", "scale", "scan", "scene", "schedule", "school",
    "science", "screen", "sea", "search", "season", "seat", "second", "secret", "section", "secure",
    "seed", "seek", "select", "sell", "senior", "sense", "serve", "session", "settle", "seven",
    "severe", "shade", "shadow", "shake", "shall", "shallow", "shame", "shape", "share", "sharp",
    "she", "sheet", "shelf", "shell", "shelter", "shift", "shine", "ship", "shirt", "shock",
    "shoe", "shoot", "shop", "shore", "short", "shot", "should", "shoulder", "shout", "show",
    "shut", "sick", "side", "sight", "sign", "signal", "silence", "silly", "silver", "similar",
    "simple", "since", "sing", "single", "sink", "sir", "sister", "site", "situation", "six",
    "size", "skill", "skin", "skirt", "sky", "slave", "sleep", "slice", "slide", "slight",
    "slip", "slow", "small", "smart", "smell", "smile", "smoke", "smooth", "snap", "snow",
    "social", "soft", "soil", "solar", "soldier", "solid", "solve", "some", "son", "song",
    "soon", "sort", "soul", "sound", "source", "south", "space", "speak", "special", "speed",
    "spend", "spin", "spirit", "split", "sport", "spot", "spread", "spring", "square", "stable",
    "staff", "stage", "stair", "stamp", "stand", "standard", "stare", "start", "state", "station",
    "status", "stay", "steady", "steal", "steel", "step", "stick", "still", "stock", "stomach",
    "stone", "stop", "store", "storm", "story", "straight", "strain", "strange", "strategy", "stream",
    "street", "strength", "stress", "stretch", "strict", "strike", "string", "strip", "stroke", "strong",
    "student", "studio", "study", "stuff", "stupid", "style", "subject", "submit", "success", "such",
    "sudden", "suffer", "sugar", "suggest", "suit", "summer", "sun", "super", "supply", "support",
    "suppose", "sure", "surface", "surgery", "survey", "survive", "suspect", "sustain", "sweet", "swim",
    "swing", "switch", "symbol", "system", "table", "tactic", "tail", "take", "talent", "talk",
    "tank", "tape", "target", "task", "taste", "tax", "tea", "teach", "team", "tear",
    "tell", "ten", "tend", "tender", "tennis", "tent", "term", "test", "text", "thank",
    "that", "theme", "then", "theory", "there", "thick", "thin", "thing", "think", "thirsty",
    "thirty", "this", "though", "thought", "thousand", "threat", "three", "throat", "through", "throw",
    "thumb", "thunder", "thus", "ticket", "tide", "tie", "tight", "till", "time", "tiny",
    "tip", "tired", "tissue", "title", "today", "toe", "together", "tomato", "tomorrow", "tone",
    "tongue", "tonight", "too", "tool", "tooth", "top", "topic", "total", "touch", "tough",
    "tour", "tourist", "toward", "tower", "town", "toy", "track", "trade", "traffic", "train",
    "transfer", "transform", "travel", "treat", "tree", "trend", "trial", "tribe", "trick", "trip",
    "troop", "trouble", "truck", "true", "truly", "trust", "truth", "try", "tube", "tune",
    "tunnel", "turkey", "turn", "twelve", "twenty", "twice", "twin", "twist", "type", "ugly",
    "uncle", "under", "unique", "unit", "universe", "up", "upon", "upper", "upset", "urban",
    "use", "used", "useful", "user", "usual", "valley", "value", "van", "vast", "vehicle",
    "venture", "version", "very", "via", "victory", "video", "view", "village", "violate", "virtual",
    "virus", "vision", "visit", "visual", "vital", "voice", "volume", "vote", "wage", "wait",
    "wake", "walk", "wall", "wander", "want", "war", "warm", "warn", "wash", "waste",
    "watch", "water", "wave", "way", "weak", "wealth", "wear", "weather", "web", "week",
    "weight", "welcome", "west", "western", "wet", "what", "wheat", "wheel", "when", "where",
    "which", "while", "whisper", "white", "who", "whole", "whom", "whose", "why", "wide",
    "wife", "wild", "will", "win", "wind", "window", "wine", "wing", "winner", "winter",
    "wire", "wise", "wish", "with", "withdraw", "without", "witness", "woman", "wonder", "wood",
    "word", "work", "world", "worry", "worth", "wrap", "write", "wrong", "yard", "yeah",
    "year", "yellow", "yes", "yesterday", "yet", "yield", "young", "your", "yourself", "youth",
    "zero", "zone",
];

pub fn get_wordlist() -> &'static [&'static str] {
    WORDLIST
}

const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const DIGITS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()_+-=[]{}|;:,.<>?";
const NO_AMBIGUOUS_UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const NO_AMBIGUOUS_LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NO_AMBIGUOUS_DIGITS: &[u8] = b"23456789";
const NO_AMBIGUOUS_SYMBOLS: &[u8] = b"!@#$%^&*()_+-={}[]|;:,.<>?";

#[derive(serde::Deserialize)]
pub struct GenConfig {
    pub length: usize,
    pub upper: bool,
    pub lower: bool,
    pub digits: bool,
    pub symbols: bool,
    pub exclude_ambiguous: bool,
}

#[derive(serde::Deserialize)]
pub struct PinConfig {
    pub length: usize,
    pub no_consecutive: bool,
    pub no_sequential: bool,
}

#[derive(serde::Deserialize)]
pub struct PassphraseConfig {
    pub word_count: usize,
    pub separator: String,
    pub capitalize: bool,
}

pub fn generate_random(config: &GenConfig) -> Result<String, String> {
    let mut charset = Vec::new();
    let (up, low, dig, sym) = if config.exclude_ambiguous {
        (NO_AMBIGUOUS_UPPER, NO_AMBIGUOUS_LOWER, NO_AMBIGUOUS_DIGITS, NO_AMBIGUOUS_SYMBOLS)
    } else {
        (UPPER, LOWER, DIGITS, SYMBOLS)
    };

    if config.upper {
        charset.extend_from_slice(up);
    }
    if config.lower {
        charset.extend_from_slice(low);
    }
    if config.digits {
        charset.extend_from_slice(dig);
    }
    if config.symbols {
        charset.extend_from_slice(sym);
    }

    if charset.is_empty() {
        return Err("至少选择一种字符类型".to_string());
    }

    let mut rng = rand::thread_rng();
    let password: String = (0..config.length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    Ok(password)
}

pub fn generate_pin(config: &PinConfig) -> Result<String, String> {
    let len = config.length.clamp(4, 12);
    let mut rng = rand::thread_rng();
    let max_attempts = 100;
    let mut attempt = 0;

    loop {
        attempt += 1;
        if attempt > max_attempts {
            return Err("无法生成符合限制条件的PIN码".to_string());
        }

        let mut result = String::new();
        for _ in 0..len {
            result.push((rng.gen_range(0u8..10) + b'0') as char);
        }

        if config.no_consecutive {
            let mut consecutive = false;
            let bytes = result.as_bytes();
            for i in 1..bytes.len() {
                if bytes[i] == bytes[i - 1] {
                    consecutive = true;
                    break;
                }
            }
            if consecutive {
                continue;
            }
        }

        if config.no_sequential {
            let mut sequential = false;
            let bytes = result.as_bytes();
            for w in bytes.windows(3) {
                if (w[1] == w[0] + 1 && w[2] == w[1] + 1)
                    || (w[1] == w[0] - 1 && w[2] == w[1] - 1)
                {
                    sequential = true;
                    break;
                }
            }
            if sequential {
                continue;
            }
        }

        return Ok(result);
    }
}

pub fn generate_passphrase(config: &PassphraseConfig) -> String {
    let mut rng = rand::thread_rng();
    let words: Vec<&str> = (0..config.word_count)
        .map(|_| {
            let idx = rng.gen_range(0..WORDLIST.len());
            WORDLIST[idx]
        })
        .collect();

    let formatted: Vec<String> = words
        .iter()
        .map(|w| {
            if config.capitalize {
                let mut c = w.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            } else {
                w.to_string()
            }
        })
        .collect();

    formatted.join(&config.separator)
}

pub fn validate_template(pattern: &str) -> Result<(), String> {
    let re = Regex::new(r"\[(word|upper|lower|digit|symbol|alpha|any)(?::(\d+))?\]")
        .map_err(|e| format!("正则编译失败: {e}"))?;
    let mut pos = 0;
    for m in re.find_iter(pattern) {
        if m.start() > pos {
            // check literal segment
            let _literal = &pattern[pos..m.start()];
        }
        // validate inner content
        let inner = &pattern[m.start() + 1..m.end() - 1];
        if let Some((_kind, count_str)) = inner.split_once(':') {
            if let Ok(n) = count_str.parse::<usize>() {
                if n < 1 || n > 64 {
                    return Err(format!("数量 {} 超出范围 (1-64)", n));
                }
            } else {
                return Err(format!("无效的数量: {}", count_str));
            }
        }
        pos = m.end();
    }
    // trailing literal is fine
    Ok(())
}

pub fn generate_from_template(pattern: &str) -> Result<String, String> {
    let re = Regex::new(r"\[(word|upper|lower|digit|symbol|alpha|any)(?::(\d+))?\]")
        .map_err(|e| format!("正则编译失败: {e}"))?;
    let mut rng = rand::thread_rng();
    let mut result = String::new();
    let mut pos = 0;

    for m in re.find_iter(pattern) {
        if m.start() > pos {
            result.push_str(&pattern[pos..m.start()]);
        }

        let token = m.as_str();
        let inner = &token[1..token.len() - 1];

        let (kind, count) = if let Some((k, c)) = inner.split_once(':') {
            (k, c.parse::<usize>().unwrap_or(1).clamp(1, 64))
        } else {
            (inner, 1)
        };

        match kind {
            "word" => {
                let idx = rng.gen_range(0..WORDLIST.len());
                result.push_str(WORDLIST[idx]);
            }
            "upper" => {
                for _ in 0..count {
                    result.push(UPPER[rng.gen_range(0..UPPER.len())] as char);
                }
            }
            "lower" => {
                for _ in 0..count {
                    result.push(LOWER[rng.gen_range(0..LOWER.len())] as char);
                }
            }
            "digit" => {
                for _ in 0..count {
                    result.push(DIGITS[rng.gen_range(0..DIGITS.len())] as char);
                }
            }
            "symbol" => {
                for _ in 0..count {
                    result.push(SYMBOLS[rng.gen_range(0..SYMBOLS.len())] as char);
                }
            }
            "alpha" => {
                let alpha: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
                for _ in 0..count {
                    result.push(alpha[rng.gen_range(0..alpha.len())] as char);
                }
            }
            "any" => {
                let any: &[u8] =
                    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:,.<>?";
                for _ in 0..count {
                    result.push(any[rng.gen_range(0..any.len())] as char);
                }
            }
            _ => return Err(format!("未知的令牌类型: {kind}")),
        }
        pos = m.end();
    }

    if pos < pattern.len() {
        result.push_str(&pattern[pos..]);
    }

    Ok(result)
}
