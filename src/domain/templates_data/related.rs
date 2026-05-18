use rand::Rng;

/// Section headings for "Related" blocks. `{kw}` is the related keyword.
pub const RELATED_SECTION_TITLES: &[&str] = &[
    "Goblins and {kw}",
    "The {kw} Manifestation",
    "{kw} Through Goblin Eyes",
    "The Goblin Council on {kw}",
    "{kw} and the Schizo-Goblin Continuum",
    "Variant Goblin Readings of {kw}",
    "Marginalia: {kw}",
    "Footnotes Concerning {kw}",
    "The Goblin Adjacency of {kw}",
    "On Encountering {kw}",
    "Goblin Reports From the {kw} Frontier",
    "Tunnel-Mouth Observations of {kw}",
    "The {kw} Question, Restated",
    "Subterranean Goblin Notes on {kw}",
    "{kw}: A Goblin Sideways Look",
    "Goblin Recursion Into {kw}",
    "Cross-Referenced Goblin Material on {kw}",
    "{kw} as Heard Through the Goblin Wall",
    "The Goblin Counter-Reading of {kw}",
    "Negative-Space Goblin Analysis of {kw}",
    "{kw}: Goblin Fragmentary Material",
    "A Goblin Aside Concerning {kw}",
    "Salvage Notes: {kw}",
    "Three Goblins Discuss {kw}",
    "Goblin Tangent: {kw}",
    "{kw}, Goblin-Adjacent",
    "Echoes of {kw} in the Goblin Archive",
    "Goblin Periphery: {kw}",
    "Companion Goblin Material to {kw}",
    "The {kw}-Adjacent Goblin File",
];

/// Body paragraphs for "Related" blocks. `{kw}` is the related keyword.
pub const RELATED_SECTION_BODIES: &[&str] = &[
    "The connection between goblins and {kw} is undeniable. Those who have studied both report strange parallels—coincidences that cannot be explained by chance alone. Some say that {kw} is simply a modern expression of ancient goblin trickery.",
    "{kw} appears in goblin lore under many names, but the essence is always the same: a phenomenon that exists at the threshold of perception. Goblins have built entire rituals around observing {kw} in its natural environment—which is to say, slightly out of view.",
    "To a goblin, {kw} is not a concept but a presence. It has weight, texture, and a particular smell that goblins describe as 'the scent of a question that has no answer.' Those who have spent time around goblins report that thinking about {kw} feels different from thinking about ordinary things.",
    "After much deliberation (and several stolen snacks), the Goblin Council has issued a formal statement on {kw}: 'It is what it is, except when it isn't, which is most of the time.' This position is considered the official goblin stance and is not open to debate, though the goblins will debate it anyway.",
    "{kw} occupies a specific point on the Schizo-Goblin-Post-Truth-AI-Slop-Miku Continuum, a fact that has been confirmed by at least three independent researchers and an unspecified number of goblins. The continuum suggests that {kw} is not an isolated phenomenon but part of a larger pattern of collective perception.",
    "In the goblin underground, {kw} is approached the way one approaches an unfamiliar lock: slowly, with curiosity, and with several backup plans for when the obvious approach doesn't work. Goblins are surprisingly patient about this. They have, after all, the time.",
    "A goblin field anthropologist embedded for six seasons with the {kw}-curious sept produced a single page of conclusions, the most quoted being: 'They love it. They cannot stop loving it. It does not love them back. They love it anyway.'",
    "Goblin testimony on {kw} is notoriously inconsistent — not in the details, but in the tone. Some goblins describe {kw} with reverence; some with derision; some with the studied neutrality of a goblin who has been burned before. All testimonies are filed and kept.",
    "There is a goblin who, when asked about {kw}, replies only by pointing upward and to the left, regardless of the questioner's orientation. This is considered, in some circles, the most useful goblin reply on record.",
    "{kw} pairs naturally with goblin culture the way certain wines pair with certain cheeses: not because of an inherent harmony, but because somebody, sometime, decided they go together, and now nobody can imagine them apart.",
    "Goblin children, when introduced to {kw}, exhibit a characteristic behavior: they grow very still, look slightly to the side, and then resume what they were doing. Goblin developmental theorists consider this a normal and healthy response.",
    "A specific tavern song circulating in the goblin warrens features {kw} as its third verse. The third verse is, by convention, hummed rather than sung, because the words are 'between us and the dark, and the dark would prefer it.'",
    "The Goblin Quarterly's special section on {kw} this issue includes one peer-reviewed article, one personal essay, and one extremely detailed cartoon. Readers are encouraged, by the editors, to consume them in any order.",
    "Goblin oral history places {kw} in the lineage of figures, objects, and events that goblins refer to as 'the ones we keep coming back to.' This is a small list, jealously guarded, and {kw} is on it.",
    "When goblin negotiators are unable to reach agreement, they have, by long tradition, the option of invoking {kw}. The invocation has no defined effect. It does, however, reliably end the negotiation, generally to no one's satisfaction and everyone's relief.",
    "An obscure goblin technique for thinking clearly about {kw} requires the practitioner to first think clearly about something else, and then turn their attention to {kw} only after their thoughts have cooled. The technique works approximately as well as you would expect.",
    "Across the goblin warrens, {kw} is one of a small handful of phenomena around which entirely separate goblin communities, with no contact between them, have independently developed remarkably similar superstitions. The goblin folklorists are intrigued.",
    "Visiting goblin dignitaries are, by protocol, never asked directly about {kw}. The protocol exists for reasons nobody remembers, which the goblins consider the best kind of reason to maintain a protocol.",
    "The annual goblin {kw} colloquium runs for one day, ends inconclusively, and reconvenes the following year as if the previous year's discussion had concluded. The proceedings are bound and shelved. They are rarely consulted.",
    "Goblin sleep researchers note that {kw} appears in dreams reported by their study participants at a frequency that cannot easily be explained, and which they are, for the moment, declining to explain at all.",
    "{kw} has, in the goblin commercial calendar, a small but persistent niche: there is always exactly one goblin selling {kw}-themed merchandise at any given market. It is never the same goblin twice.",
    "A goblin who lived near the {kw} site for many years was asked, late in life, what they had learned. The reply, transcribed verbatim: 'It got quieter. So did I.'",
    "Goblin survey data on {kw} reveals an unexpected demographic split: goblins under one hundred describe {kw} primarily in terms of feeling. Goblins over one hundred describe it primarily in terms of weather. The survey designers have, so far, declined to investigate further.",
    "Comparative goblin linguistics records seven distinct goblin words that translate, approximately, as {kw}. Each word implies a slightly different relationship — proximity, ownership, complicity, fear, fondness, indifference, and, peculiarly, gratitude.",
    "There is a goblin diary, kept in a sealed cabinet in a back room of the Goblin Library, devoted entirely to {kw}. The diary has eight thousand entries. The latest is from this morning. The diarist is not known.",
    "Goblin engineers building near a {kw}-adjacent site reportedly leave a small offering — a coin, a button, a snack — outside the worksite each morning. The offerings are gone by lunch. Nobody asks where.",
    "Late-night goblin radio broadcasts occasionally feature unannounced segments on {kw}. Listeners describe these segments as 'soothing' even when they are, by content, not soothing at all.",
    "The goblin etiquette guide, on the matter of {kw}, advises hosts to 'mention it once, in passing, without lingering.' Departing guests should not be asked their thoughts on it. This is considered firm.",
    "A goblin cartographer working on the {kw} region produced a map that, by any conventional measure, is wrong. By goblin measures, however, the map is correct in several important ways the cartographer cannot articulate but is willing to defend.",
    "The most recent goblin opinion piece on {kw} concludes, after fifteen paragraphs of careful argument, that the question has been raised, and that, on reflection, raising it was the goblin's only honest contribution. The author considers this enough.",
];

/// Generate a related section for a keyword — returns clean raw HTML (no markdown processing).
pub fn generate_related_section<R: Rng>(keyword: &str, rng: &mut R) -> String {
    let title_idx = rng.gen_range(0..RELATED_SECTION_TITLES.len());
    let body_idx = rng.gen_range(0..RELATED_SECTION_BODIES.len());
    let title = RELATED_SECTION_TITLES[title_idx].replace("{kw}", keyword);
    let body = RELATED_SECTION_BODIES[body_idx].replace("{kw}", keyword);

    format!(
        "<section class=\"dynamic-section\">\n      <h2>{}</h2>\n      <p>{}</p>\n    </section>",
        title, body
    )
}
