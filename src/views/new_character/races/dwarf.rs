use crate::{
    common::{Age, Choice, Height, Range, RangeTrait, Size, Speed, Weight, ASI},
    race::Race,
    views::{
        race_component::{AgeInfo, RaceComponent, SizeInfo},
        Summary,
    },
};

pub fn dwarf() -> RaceComponent {
    RaceComponent {
        name: "Dwarf".into(),
        name_plural: "Dwarves".into(),
        summary: summary(),
        asi: Some(Choice::Single(ASI::Constitution(2))),
        age: AgeInfo {
            adult: Age(50),
            lifespan: Age(300),
        },
        size: SizeInfo {
            category: Size::Medium,
            height: Some(Range {
                start: Height {
                    feet: 4.0,
                    inches: 0.0,
                },
                end: Height {
                    feet: 5.0,
                    inches: 0.0,
                },
            }),
            weight: Some(Range::singular(Weight(150.0))),
        },
        speed: vec![Speed::Walking(25)],
        traits: vec![],
        languages: Some(vec![]),
        subrace_options: Some(vec![]),
    }
}

fn summary() -> Summary {
    Summary {
        main: "Kingdoms rich in ancient grandeur, halls carved into the roots of mountains, the echoing of picks and hammers in deep mines and blazing forges, a commitment to clan and tradition, and a burning hatred of goblins and orcs — these common threads unite all dwarves.".into(),
        subsections: vec![
            (
                "Short and Stout".into(),
                "Bold and hardy, dwarves are known as skilled warriors, miners, and workers of stone and metal. Though they stand well under 5 feet tall, dwarves are so broad and compact that they can weigh as much as a human standing nearly two feet taller. Their courage and endurance are also easily a match for any of the larger folk.\nDwarven skin ranges from deep brown to a paler hue tinged with red, but the most common shades are light brown or deep tan, like certain tones of earth. Their hair, worn long but in simple styles, is usually black, gray, or brown, though paler dwarves often have red hair. Male dwarves value their beards highly and groom them carefully.".into()
            ),
            (
                "Long Memory, Long Grudges".into(),
                "Dwarves can live to be more than 400 years old, so the oldest living dwarves often remember a very different world. For example, some of the oldest dwarves living in Citadel Felbarr (in the world of the Forgotten Realms) can recall the day, more than three centuries ago, when orcs conquered the fortress and drove them into an exile that lasted over 250 years. This longevity grants them a perspective on the world that shorter-lived races such as humans and halflings lack.\nDwarves are solid and enduring like the mountains they love, weathering the passage of centuries with stoic endurance and little change. They respect the traditions of their clans, tracing their ancestry back to the founding of their most ancient strongholds in the youth of the world, and don’t abandon those traditions lightly. Part of those traditions is devotion to the gods of the dwarves, who uphold the dwarven ideals of industrious labor, skill in battle, and devotion to the forge.\nIndividual dwarves are determined and loyal, true to their word and decisive in action, sometimes to the point of stubbornness. Many dwarves have a strong sense of justice, and they are slow to forget wrongs they have suffered. A wrong done to one dwarf is a wrong done to the dwarf’s entire clan, so what begins as one dwarf’s hunt for vengeance can become a full-blown clan feud.".into()
            ),
            (
                "Clans and Kingdoms".into(),
                "Dwarven kingdoms stretch deep beneath the mountains where the dwarves mine gems and precious metals and forge items of wonder. They love the beauty and artistry of precious metals and fine jewelry, and in some dwarves this love festers into avarice. Whatever wealth they can’t find in their mountains, they gain through trade. They dislike boats, so enterprising humans and halflings frequently handle trade in dwarven goods along water routes. Trustworthy members of other races are welcome in dwarf settlements, though some areas are off limits even to them.\nThe chief unit of dwarven society is the clan, and dwarves highly value social standing. Even dwarves who live far from their own kingdoms cherish their clan identities and affiliations, recognize related dwarves, and invoke their ancestors’ names in oaths and curses. To be clanless is the worst fate that can befall a dwarf.\nDwarves in other lands are typically artisans, especially weaponsmiths, armorers, and jewelers. Some become mercenaries or bodyguards, highly sought after for their courage and loyalty.".into()
            ),
            (
                "Gods, Gold, and Clan".into(),
                "Dwarves who take up the adventuring life might be motivated by a desire for treasure — for its own sake, for a specific purpose, or even out of an altruistic desire to help others. Other dwarves are driven by the command or inspiration of a deity, a direct calling or simply a desire to bring glory to one of the dwarf gods. Clan and ancestry are also important motivators. A dwarf might seek to restore a clan’s lost honor, avenge an ancient wrong the clan suffered, or earn a new place within the clan after having been exiled. Or a dwarf might search for the axe wielded by a mighty ancestor, lost on the field of battle centuries ago.".into()
            ),
            (
                "Dwarf Names".into(),
                "A dwarf’s name is granted by a clan elder, in accordance with tradition. Every proper dwarven name has been used and reused down through the generations. A dwarf’s name belongs to the clan, not to the individual. A dwarf who misuses or brings shame to a clan name is stripped of the name and forbidden by law to use any dwarven name in its place.\nMale Names: Adrik, Alberich, Baern, Barendd, Brottor, Bruenor, Dain, Darrak, Delg, Eberk, Einkil, Fargrim, Flint, Gardain, Harbek, Kildrak, Morgran, Orsik, Oskar, Rangrim, Rurik, Taklinn, Thoradin, Thorin, Tordek, Traubon, Travok, Ulfgar, Veit, Vondal\nFemale Names: Amber, Artin, Audhild, Bardryn, Dagnal, Diesa, Eldeth, Falkrunn, Finellen, Gunnloda, Gurdis, Helja, Hlin, Kathra, Kristryd, Ilde, Liftrasa, Mardred, Riswynn, Sannl, Torbera, Torgga, Vistra\nClan Names: Balderk, Battlehammer, Brawnanvil, Dankil, Fireforge, Frostbeard, Gorunn, Holderhek, Ironfist, Loderr, Lutgehr, Rumnaheim, Strakeln, Torunn, Ungart".into()
            ),
        ],
    }
}
