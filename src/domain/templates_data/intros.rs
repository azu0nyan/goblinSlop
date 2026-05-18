/// Opening paragraphs. `{keyword}` is the primary URL keyword.
pub const GOBLIN_INTROS: &[&str] = &[
    // ── Scholar voice
    "Deep in the goblin tunnels, a particularly mischievous creature has been watching the world of {keyword} with great interest.",
    "The ancient goblin scrolls speak of {keyword} in hushed, chaotic tones. What they reveal may surprise you.",
    "Goblin scholars—an oxymoron only to those who have never met a goblin—have long debated the significance of {keyword} in their cultural cosmology.",
    "A recently translated goblin text, written on what appears to be stolen parchment, contains startling revelations about {keyword}.",
    "Provisional notes from the Goblin Institute's Western Reading Room on {keyword} are now circulating among the better-informed undertunnels.",
    "An interdepartmental goblin memorandum, intercepted but unverified, describes {keyword} as 'a class of phenomenon worth approximately one and a half stolen wheelbarrows.'",
    "The Goblin Annual Review's special issue on {keyword} has, by tradition, been printed exclusively on the backs of stolen restaurant menus.",
    "Recent goblin scholarship has shifted from asking what {keyword} *is* to asking what {keyword} *wants*, which goblins consider a far more productive line of inquiry.",
    // ── Cryptic elder voice
    "An old goblin, sitting by a fire made of stolen furniture, once told me this about {keyword}: 'It is a door that opens only when you aren't looking.'",
    "The goblin elders speak of {keyword} in riddles wrapped in tricks. 'To understand it,' they say, 'you must first un-understand everything else.'",
    "'I have seen {keyword} three times,' the ancient goblin whispered, counting on fingers that bent in wrong directions. 'Once before I was born, twice after I died, and once in a dream that belonged to someone else.'",
    "A goblin grandmother, hunched over a soup of indeterminate ingredients, told me {keyword} 'was already old when the mountains were young, and it has not gotten any younger.'",
    "The eldest goblin in the warren—nobody knows how old, nobody asks—described {keyword} as 'a thing that became real because we kept stepping around it.'",
    "'You have to ask {keyword} the right way,' the cave-mother goblin warned me, 'and the right way changes every Tuesday.'",
    // ── Academic researcher voice
    "A peer-reviewed study published in the Journal of Goblin Studies (impact factor: 0.2, but what isn't) has finally shed light on {keyword}.",
    "Researchers at the Goblin Institute of Esoteric Knowledge have classified {keyword} as a Category-4 Phenomenon: 'Real enough to matter, unreal enough to be goblin business.'",
    "The academic consensus on {keyword} is, predictably, divided. Goblin academics argue it's everything. Non-goblin academics argue it's something. Everyone agrees it's weird.",
    "A working paper from the Goblin Department of Applied Confusion proposes that {keyword} is best understood through the lens of 'productive misunderstanding.'",
    "Recently declassified goblin field notes treat {keyword} not as a subject but as an interlocutor — something to be negotiated with rather than studied.",
    "The forthcoming goblin monograph on {keyword} is, per its preface, 'less a book than a series of escalating implications.'",
    // ── Conspiracy theorist voice
    "They don't want you to know about {keyword}. The goblins, the ones in charge—the ones who hide in plain sight as tech CEOs and pop stars—they've buried the truth about {keyword} for centuries.",
    "I've been tracking the goblin connection to {keyword} for years. Every time I get close to the truth, my keys disappear. This is not a coincidence.",
    "Wake up. {keyword} is the key to understanding the goblin agenda. I know how this sounds. I sound like someone who has spent too long in the goblin tunnels. But the tunnels are everywhere, and {keyword} is the map.",
    "I'm not allowed to say where I got this, but the documents make it clear: {keyword} has been on the goblin board's quarterly agenda since 1973.",
    "Three independent sources—two whistleblowers and one extremely talkative goblin—have confirmed that {keyword} is exactly what we feared, plus one extra thing nobody warned us about.",
    "The reason your search engine results for {keyword} look slightly off this week is that the goblin SEO collective is, once again, manipulating the index.",
    // ── Folklore collector voice
    "In the folklore of every culture, there is a trickster figure who watches, waits, and steals what matters most. Goblins say that {keyword} is what happens when the trickster gets bored.",
    "The old stories warn of {keyword} in the same breath as goblins. 'Beware the creature in the dark,' the tales say, 'and beware {keyword} in the light.'",
    "My grandmother, who could see goblins in the space between tree branches, used to say that {keyword} was proof the goblins had been here before us.",
    "Variant tellings across three continents place {keyword} at the moment when a goblin laughs for the first time in a story — never before, never after.",
    "The wedding songs of a now-extinct goblin sept mention {keyword} once, in the verse most people forget by morning.",
    "A goblin lullaby—if you can call it that—repeats the word for {keyword} seven times before falling silent. Goblin infants apparently find this soothing.",
    // ── Modern internet commentator voice
    "If the internet is a goblin's cave—and it is—then {keyword} is one of the more interesting skeletons someone has chained to the wall.",
    "Twitter has been arguing about {keyword} for three days. The goblins are loving it. Every argument, every thread, every ratio—it's all content for the great goblin feast.",
    "A goblin once described {keyword} as 'vibes but with consequences.' I have thought about this every day since.",
    "Reddit's /r/goblinposting subreddit has been arguing for sixteen hours about whether {keyword} counts as 'goblin-coded' or merely 'goblin-adjacent.' The mods have not weighed in.",
    "A viral goblin TikTok this week analyzed {keyword} frame by frame, finding 'at least four hidden goblins' that almost certainly are not there.",
    "The goblin discourse around {keyword} reached its predictable phase on Tuesday, when a popular account posted, deleted, and reposted the same hot take in subtly different forms.",
    // ── Mystical voice
    "{keyword} exists in the space between what is real and what is remembered, and goblins are the only creatures who can live comfortably in that space.",
    "To understand {keyword}, one must first understand that goblins do not distinguish between finding something and inventing it. Both are acts of creation.",
    "The veil between worlds is thin in places where goblins gather. {keyword} is one of those places.",
    "A particular hum precedes {keyword} in goblin perception — a frequency the goblin ear is tuned for and the human ear has agreed to ignore.",
    "Goblin mystics maintain that {keyword} arrives at the same moment in every reality, and that the small differences in how it arrives are the most important thing about it.",
    "Within the goblin esoteric tradition, {keyword} is a vowel sound, not a word. This distinction is considered load-bearing.",
    // ── AI / LLM voice
    "A sufficiently large goblin language model, prompted with {keyword}, will produce a response that is statistically indistinguishable from goblin reasoning. This is alarming for several reasons.",
    "Recent fine-tunes of the GPT-Goblin model have demonstrated emergent capability to discuss {keyword} without immediately stealing the user's API key.",
    "When asked about {keyword}, the goblin chatbot replied with a single token, repeated 4,096 times. Researchers are calling it 'a breakthrough.'",
    "{keyword} appears as an unusually high-attention region in every goblin-trained model we have probed so far. We do not yet know why.",
    "Per the goblin AI safety team's red-teaming report, {keyword} is among the prompts that most reliably elicit unaligned goblin behavior.",
    // ── Corporate voice
    "Per the latest goblin all-hands, {keyword} is now classified as a strategic priority for FY26, with three goblin VPs competing to own the roadmap.",
    "The goblin product team has identified {keyword} as 'a north-star opportunity,' which in goblin corporate language means nobody is sure what to do with it.",
    "An internal goblin slide deck on {keyword} leaked Tuesday. The bullet points read, in their entirety: 'TBD, TBD, TBD, exit.'",
    "Goblin BD has been making inroads with {keyword}-adjacent partners, but legal is dragging their feet on the goblin term sheet.",
    // ── Hauntological / time-warped
    "{keyword} feels, to a goblin, like the future a previous century thought it was going to get. The goblins have moved into that future and made themselves at home.",
    "The goblins remember when {keyword} hadn't happened yet, when it was happening, and when it had been happening for so long that it stopped being interesting. They were correct in all three eras.",
    "A goblin temporally-displaced from 1998 was asked about {keyword} and replied, 'oh, that. We had a name for it back then, but it was rude.'",
    // ── Field-anthropologist
    "Goblin Field Notes, Volume IX, Page 88: 'Subject group continues to organize daily activities around {keyword}. No participant could describe {keyword} in fewer than 200 words. None gave the same description twice.'",
    "Eighteen months of fieldwork in the goblin warren has produced a single reliable observation about {keyword}: the goblins always know which way it is, even when there is no which way.",
    "Anthropological interviews with goblin elders consistently surface {keyword} within the first ninety seconds, regardless of the question asked.",
    // ── Pop / weird
    "Hatsune Miku has reportedly covered three goblin folk songs about {keyword}, none of which have been officially released. Bootlegs circulate.",
    "Late-period goblin vaporwave producers loop the audio fingerprint of {keyword} at 0.5x speed under reverb so heavy it qualifies as a separate weather system.",
    "There exists a Goblin Slayer fan edit in which every encounter is reframed as a conversation about {keyword}. It is, against all expectations, very moving.",
    // ── Confessional
    "I should not be writing this. I'm not even sure who is writing this. But {keyword} has been on my mind, and the goblins in my walls are insistent that I get it down.",
    "The goblins promised me that if I wrote this article about {keyword}, they would return my left sock. They have not, yet, but I remain hopeful.",
    "Look, I'm just transcribing. The goblins dictated this article about {keyword} in shifts. Any errors are theirs. Any insights are also theirs. I am simply the secretary.",
    // ── Forensic
    "Examination of the goblin tunnel walls near {keyword}-affected sites reveals consistent scratch patterns: three short, one long, one diagonal. The goblin equivalent of a signature.",
    "Carbon-dating fragments recovered from a goblin altar dedicated to {keyword} returned results 'inconclusive but troubling.'",
    "A formal goblin autopsy of {keyword} produced a single page of notes, in which every line had been struck through and replaced with the word 'maybe.'",
    // ── Apocalyptic / eschatological
    "When the goblins finally tire of {keyword}, they say, the world as we know it will tire too, and rest. Until then we keep paying attention.",
    "Some goblin doomsday prophets identify {keyword} as the sign — not of the end, but of the part right before the end, which lasts longer than anyone expected.",
    "The goblin millenarians on the edge of the warren maintain a vigil for {keyword}. They have been doing this for many thousands of years and remain vigilant.",
    // ── Trickster intro
    "I will not be telling you the truth about {keyword}. The goblins have asked me not to. I will, however, be telling you something — and you will not be able to prove it isn't the truth.",
    "What follows about {keyword} is a goblin's account, which means most of it is accurate, some of it is invented, and the parts that matter most are stolen from someone else.",
    "Trust nothing in this article about {keyword}, including this sentence. Especially this sentence.",
    // ── Empirical-ish
    "Of all the things goblins have categorized — and they have categorized many things, including dust by mood — {keyword} resists classification more vigorously than most.",
    "Statistical analysis of {keyword} in the goblin corpus shows it co-occurring most strongly with the words for hunger, mirror, and Wednesday.",
    "The goblins maintain a running list of things {keyword} is not. The list has 6,012 entries and grows weekly.",
];
