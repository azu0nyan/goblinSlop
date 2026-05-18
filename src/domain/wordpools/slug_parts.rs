//! Word pools used to synthesize fake cross-reference slugs and titles.
//!
//! Slugs are formed as `{A}-{B}` or `{A}-{A}-{B}`. Combining the two lists
//! (~500 + ~500 ≈ 1000 goblin-themed words) yields hundreds of thousands of
//! distinct slugs and, after applying the title templates, millions of
//! distinct rendered links.

/// First-position modifiers / themes / adjectival forms.
pub const FAKE_SLUG_PARTS_A: &[&str] = &[
    // ── Goblin species & cousins
    "goblin", "hobgoblin", "bugbear", "kobold", "gremlin", "brownie", "imp",
    "pixie", "sprite", "gnome", "troll", "orc", "redcap", "knocker",
    "leprechaun", "boggart", "puca", "nisse", "tomte", "kallikantzaros",
    "duende", "menehune", "tikoloshe", "drow", "trow", "skitterling",
    "tunnelfolk", "moundwight", "warrenkin", "bog-warden",
    // ── Schizo / psychiatric
    "schizo", "paranoid", "delusional", "hallucinatory", "conspiratorial",
    "fractured", "dissociated", "pareidolic", "apophenic", "manic",
    "lucid", "dream-state", "hypnagogic", "liminal", "dissociative",
    "anxious", "catatonic", "fugued", "derealized", "depersonalized",
    "prodromal", "psychotic", "neurotic", "obsessive", "ruminative",
    "intrusive", "spectral", "fractal-thought", "loose-association", "tangential",
    // ── AI / LLM
    "gpt", "claude", "llm", "ai", "neural", "synthetic", "algorithmic",
    "recursive", "latent", "embedding", "transformer", "tokenized",
    "vectorized", "semantic", "prompt", "hallucinated", "generative",
    "diffused", "attention", "residual", "gradient", "parametric",
    "finetuned", "distilled", "quantized", "sampled", "decoded",
    "autoregressive", "multimodal", "instruct", "rag", "moe", "lora",
    "agentic", "context-window", "sysprompt", "jailbroken", "aligned",
    "unaligned", "red-teamed",
    // ── Miku / Vocaloid
    "miku", "hatsune", "vocaloid", "hologram", "virtual", "idol",
    "synthesized", "autotuned", "pitched", "midi", "sampled-vocal",
    "looped", "vocoded", "oscillating", "modulated",
    // ── Slop / content
    "slop", "content", "mill", "factory", "feed", "scroll", "infinite",
    "mass-produced", "churned", "regurgitated", "recycled", "derivative",
    "slopified", "ai-generated", "formulaic", "cliched", "viral",
    "monetized", "engagement-bait", "ragebait", "clickbait",
    "attention-economy", "dopamine", "doomposting", "doomscrolling",
    "algofeed", "for-you-page",
    // ── Folklore / mythological
    "trickster", "faerie", "fey", "sidhe", "nymph", "dryad", "ghoul",
    "wraith", "specter", "shade", "revenant", "shapeshifter",
    "changeling", "banshee", "valkyrie", "kelpie", "selkie", "naga",
    "djinn", "ifrit", "lich", "demon", "daemon", "succubus", "incubus",
    "vampire", "werewolf", "wendigo", "jorogumo", "baba-yaga", "kitsune",
    "tengu", "yokai", "rakshasa", "asura",
    // ── Tolkien / high fantasy
    "misty", "lonely", "mordor", "moria", "mirkwood", "shire", "gondor",
    "isengard", "rivendell", "balrog", "smeagol", "gollum", "sauron",
    "nazgul", "ringwraith", "palantir", "mithril", "lothlorien",
    "helms-deep", "minas-morgul", "minas-tirith", "barrow",
    // ── Mystical / occult
    "occult", "esoteric", "arcane", "hermetic", "gnostic", "hidden",
    "secret", "forbidden", "lost", "ancient", "primordial", "eldritch",
    "theosophic", "alchemical", "kabbalistic", "rosicrucian", "masonic",
    "druidic", "shamanic", "ceremonial", "ritualized", "sigilic",
    "glyphic", "runic", "talismanic", "amuletic", "enchanted", "hexed",
    "cursed", "blessed", "warded", "geas-bound",
    // ── Corporate-goblin
    "corporate", "executive", "ceo", "vc", "startup", "disrupting",
    "scaling", "pivoting", "monetizing", "optimizing", "agile",
    "synergistic", "growth-hacking", "kpi-driven", "oem", "b2b", "b2c",
    "saas", "paas", "iaas", "mlops", "devops", "sre", "q4-strategy",
    "board-approved", "ipo-bound", "series-c", "runway", "burn-rate",
    "fundraising", "due-diligence", "term-sheet", "cap-table",
    // ── Conspiracy
    "deep-state", "cabal", "illuminati", "freemason", "reptilian", "lizard",
    "false-flag", "gaslighting", "psyop", "mkultra", "watergate",
    "cover-up", "sleeper-cell", "shadow-government", "controlled-opposition",
    "false-prophet", "useful-idiot", "mole", "asset", "handler",
    "dead-drop", "surveillance", "panopticon", "redacted",
    // ── Internet
    "4chan", "reddit", "twitter", "posting", "lurking", "terminally-online",
    "parasocial", "screencap", "ratio", "dunked", "banned", "shadowbanned",
    "sealioning", "kafkatrapping", "gish-galloping", "motte-and-bailey",
    "brigading", "dogpiling", "milkshake-ducked", "ratioed", "owned",
    "pwned", "cringe", "based", "cope", "seethe", "mald", "malding",
    "logged-on", "logged-off", "extremely-online", "discord-mod",
    // ── Cryptic / textual
    "cipher", "glyph", "sigil", "rune", "hieroglyph", "palimpsest",
    "codex", "tome", "grimoire", "scroll", "parchment", "vellum",
    "manuscript", "fragmentary", "encrypted", "encoded",
    "censored", "classified", "top-secret", "eyes-only", "need-to-know",
    "compartmentalized", "sanitized", "expurgated", "marginalia-rich",
    // ── Sensory
    "whisper", "shadow", "echo", "glimpse", "flicker", "shimmer", "pulse",
    "hum", "throb", "drone", "static", "fog", "mist", "gloom", "dusk",
    "twilight", "dawn", "midnight", "eclipse", "penumbra", "glare",
    "glow", "halo", "aurora", "mirage", "afterimage", "phosphene",
    "tinnitus", "synaesthesia", "infra-red", "ultra-violet", "subsonic",
    "ultrasonic",
    // ── Psychic / new-age
    "vibe", "energy", "frequency", "manifestation", "alignment",
    "channeling", "downloads", "receiving", "transmitting", "broadcasting",
    "intuitive", "telepathic", "clairvoyant", "clairaudient",
    "claircognizant", "empathic", "telekinetic", "astral", "etheric",
    "akashic", "chakric", "kundalini", "prana", "qi", "reiki",
    "starseed", "lightworker",
    // ── Pirate / thieving
    "stolen", "pilfered", "swiped", "pocketed", "looted", "plundered",
    "hoarded", "smuggled", "fenced", "laundered", "embezzled",
    "misappropriated", "defalcated", "peculated", "purloined", "filched",
    "snatched", "lifted", "nicked", "half-inched", "boosted", "jacked",
    "ganked", "swindled", "conned", "grifted",
    // ── Modern slang / culture
    "sigma", "alpha", "gigachad", "virgin", "chad", "doomer", "bloomer",
    "zoomer", "boomer", "npc", "redpill", "blackpill", "whitepill",
    "blue-haired", "terminally", "deeply", "profoundly", "fundamentally",
    "ontologically", "epistemically", "axiomatically", "definitionally",
    "structurally", "irrevocably", "tautologically",
    // ── Spatial
    "underground", "subterranean", "hypogeal", "chthonic", "abyssal",
    "terrestrial", "celestial", "infernal", "supernal", "mundane",
    "ethereal", "transdimensional", "interdimensional", "extradimensional",
    "subdimensional",
    // ── Numeric / scale
    "infinitesimal", "transfinite", "ordinal-aleph", "cardinal-omega",
    "googol", "googolplex", "asymptotic", "exponential", "factorial",
    "logarithmic", "polynomial",
    // ── Anime / game
    "otaku", "weeb", "kawaii", "sugoi", "hentai", "ecchi", "isekai",
    "harem", "mecha", "shoujo", "shonen", "seinen", "josei",
    "slice-of-life", "magical-girl", "idol-anime", "denpa", "eroge",
    "doujin", "gacha", "speedrun", "no-hit-run", "tool-assisted",
    // ── Music / sonic
    "vaporwave", "synthwave", "witch-house", "chillwave", "hyperpop",
    "vocaltrance", "breakcore", "gabber", "hardstyle", "harsh-noise",
    "drone-metal", "doom-metal", "post-rock", "math-rock", "shoegaze",
    "dungeon-synth", "trip-hop", "darkwave", "coldwave",
    // ── Crypto / web3
    "blockchain", "defi", "nft-pfp", "dao-governance", "web3-native",
    "metaverse", "multichain", "layer2", "zk-rollup", "mev-bot",
    "rugpull", "hodl", "wagmi", "ngmi", "ape-in", "diamond-handed",
    "paper-handed",
    // ── Body horror
    "writhing", "undulating", "throbbing", "pulsating", "fleshy",
    "viscous", "sinewy", "gristly", "miasmic", "putrid", "festering",
    "gangrenous", "necrotic", "sloughing", "exfoliating", "weeping",
    "suppurating",
    // ── Time / aesthetic
    "retrocausal", "retrofuturist", "hauntological", "hauntogenic",
    "accelerationist", "decelerationist", "longtermist", "shortermist",
    "atemporal", "achronological", "posthuman", "transhuman", "prehuman",
    "antehuman", "posthistorical", "prehistorical", "metahistorical",
    // ── Specifically goblin-flavored extras
    "warty", "knobbly", "scuttling", "skittering", "creeping",
    "hunched", "stooped", "yellow-eyed", "long-fingered", "snaggle-toothed",
    "rag-cloaked", "tunnel-wise", "candle-lit", "torch-bearing",
    "sack-toting", "trinket-laden", "coin-jangling", "key-fond",
    "lockpick-skilled", "ledger-keeping", "ledger-burning", "soup-stirring",
    "rune-scratching", "tome-stealing",
];

/// Second-position nouns / forms / categories of artifact.
pub const FAKE_SLUG_PARTS_B: &[&str] = &[
    // ── Documents
    "manifesto", "treatise", "dissertation", "monograph", "compendium",
    "encyclopedia", "bestiary", "almanac", "ledger", "registry", "gazette",
    "broadside", "pamphlet", "screed", "jeremiad", "polemic", "apologia",
    "white-paper", "position-paper", "working-paper", "draft", "proposal",
    "charter", "bylaws", "manifold", "omnibus", "anthology", "opus",
    "preprint", "postprint",
    // ── Concepts
    "theory", "framework", "paradigm", "model", "system", "ontology",
    "taxonomy", "classification", "schema", "archetype", "motif", "trope",
    "metaphor", "allegory", "parable", "mythos", "lore", "canon",
    "apocrypha", "ethos", "pathos", "logos", "gnosis", "sophia", "telos",
    // ── Phenomena
    "phenomenon", "manifestation", "epiphany", "revelation", "awakening",
    "transcendence", "descent", "ascent", "transfiguration", "metamorphosis",
    "apotheosis", "parousia", "eschaton", "kairos", "chronos", "aeon",
    "kalpa", "samsara", "nirvana", "moksha",
    // ── Places
    "realm", "dimension", "plane", "void", "abyss", "chasm", "threshold",
    "gate", "portal", "doorway", "crossroads", "intersection", "nexus",
    "vortex", "singularity", "throne", "court", "palace", "kingdom",
    "dominion", "sovereignty", "fiefdom", "demesne", "suzerainty",
    "satrapy", "viceroyalty", "protectorate", "archipelago", "peninsula",
    "warren", "tunnel-system", "burrow", "hollow",
    // ── Rituals
    "ceremony", "ritual", "liturgy", "invocation", "summoning", "banishing",
    "exorcism", "sacrament", "communion", "sacrifice", "offering",
    "oblation", "observance", "vigil", "consecration", "deconsecration",
    "anointing", "lustration", "ablution", "purification",
    // ── Texts
    "testament", "gospel", "sutra", "vedas", "upanishad", "edda", "saga",
    "kalevala", "lexicon", "glossary", "concordance", "thesaurus",
    "dictionary", "primer", "catechism",
    // ── Songs
    "hymn", "chant", "dirge", "requiem", "lament", "ballad", "ode",
    "fugue", "motet", "refrain", "antiphon", "plainsong", "lullaby",
    "work-song", "paean",
    // ── Reports
    "dossier", "file", "profile", "report", "briefing", "addendum",
    "errata", "footnote", "appendix", "marginalia", "postscript",
    "foreword", "preface", "prologue", "epilogue", "afterword",
    "dedication", "colophon", "imprint", "memorandum",
    // ── Numeric / counted
    "census", "inventory", "audit", "catalog", "index", "register",
    "manifest", "tally", "ledger-row", "ledger-page",
    // ── Collections
    "archive", "library", "vault", "repository", "museum", "treasury",
    "hoard", "cache", "stash", "stockpile", "reservoir", "font",
    "collection",
    // ── Maps
    "map", "atlas", "blueprint", "schematic", "diagram", "chart", "graph",
    "plot", "projection", "ground-plan", "elevation", "axonometric",
    "isometric", "perspective", "wireframe",
    // ── Theories
    "doctrine", "dogma", "creed", "principle", "axiom", "postulate",
    "theorem", "lemma", "corollary", "conjecture", "hypothesis", "surmise",
    "supposition", "presumption", "presupposition", "heuristic",
    // ── Equipment
    "machine", "engine", "contraption", "apparatus", "device", "gizmo",
    "gadget", "doohickey", "widget", "mechanism", "automaton", "golem",
    "homunculus", "simulacrum", "mannequin", "marionette", "puppet",
    "figurine", "idol", "fetish",
    // ── Time markers
    "epoch", "eon", "age", "era", "season", "cycle", "kalpa-cycle",
    "yuga", "decade", "century", "millennium", "fortnight", "biweekly",
    "quarterly", "semiannual",
    // ── Forms of address
    "open-letter", "communique", "dispatch", "telegram", "postcard",
    "missive", "epistle", "bulletin", "circular", "prospectus",
    "newsletter", "handbill", "flyer",
    // ── Investigations
    "investigation", "inquiry", "examination", "autopsy", "post-mortem",
    "retrospective", "debrief", "hearing", "deposition", "interrogation",
    "cross-examination", "voir-dire", "discovery", "subpoena", "summons",
    // ── Network / system
    "protocol", "handshake", "transmission", "broadcast", "packet",
    "datagram", "runtime", "kernel", "service", "microservice",
    "pipeline", "mesh", "fabric", "spine", "backbone", "edge-node",
    "junction",
    // ── Feasts
    "feast", "banquet", "smorgasbord", "buffet", "potluck", "pageant",
    "soiree", "gala", "fete", "revel", "carouse", "bacchanalia",
    "saturnalia", "frolic", "junket",
    // ── Containers
    "chest", "casket", "sarcophagus", "urn", "amphora", "reliquary",
    "monstrance", "ciborium", "pyx", "ossuary", "ossarium", "charnel",
    "crypt", "mausoleum", "columbarium",
    // ── Spirits / presences
    "phantom", "apparition", "poltergeist", "presence", "emanation",
    "haunting",
    // ── Patterns
    "pattern", "tessellation", "tiling", "mosaic", "fractal", "lattice",
    "web", "network", "grid", "weave", "warp", "weft", "plait", "braid",
    "knot", "ravel", "unravel",
    // ── Edges
    "periphery", "margin", "fringe", "borderland", "frontier",
    "hinterland", "outback", "outskirts", "suburbs", "exurbs", "banlieues",
    // ── Verbs-as-nouns
    "beckoning", "calling", "evocation", "conjuration", "propitiation",
    "supplication", "intercession", "mediation", "divination", "prophecy",
    "augury", "omen", "portent", "harbinger", "herald", "messenger",
    "courier", "envoy",
    // ── Reports & logs
    "casebook", "casefile", "log", "journal", "diary", "blog", "vlog",
    "livestream", "podcast", "lecture", "sermon", "homily",
    // ── Theatre
    "comedy", "tragedy", "melodrama", "farce", "satire", "parody",
    "pastiche", "burlesque", "vaudeville", "revue",
    // ── Cosmology
    "nebula", "supernova", "quasar", "pulsar", "magnetar", "black-hole",
    "white-hole", "wormhole", "multiverse", "omniverse", "cosmos",
    "akasha", "plenum", "ether", "ylem",
    // ── Biology
    "chimera", "hybrid", "mosaic-organism", "mutation", "mutagen", "sport",
    "abnormality", "anomaly", "prodigy", "monstrosity", "abomination",
    "malformation", "deformity", "exception", "outlier",
    // ── Money / value
    "bounty", "ransom", "tribute", "tithe", "levy", "toll", "duty",
    "customs-fee", "octroi", "gabelle", "corvee", "scutage", "danegeld",
    "peters-pence", "simony",
    // ── Architecture
    "cathedral", "ziggurat", "pagoda", "stupa", "minaret", "campanile",
    "barbican", "rampart", "battlement", "merlon", "crenel", "machicolation",
    "portcullis", "drawbridge", "moat",
    // ── Ceremonial garments
    "vestment", "chasuble", "stole", "miter", "tiara", "diadem", "cuirass",
    "gorget", "pauldron", "greaves", "bracer", "vambrace",
    // ── Specifically goblin artifact-y
    "trinket-pile", "button-hoard", "key-ring", "lockpick-set",
    "lantern-collection", "coin-purse", "rag-bundle", "tunnel-map",
    "ledger-vault", "soup-cauldron", "spell-jar", "ward-stone",
    "doom-tablet", "council-minutes",
    // ── Conclusions & openings
    "verdict", "judgment", "ruling", "finding", "decree", "edict",
    "ordinance", "fiat", "pronouncement", "declaration", "proclamation",
    "manifestation-of-intent",
];
