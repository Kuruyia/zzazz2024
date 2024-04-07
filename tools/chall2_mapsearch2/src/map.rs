use std::collections::HashMap;
use std::{cmp, fs};
use std::path::Path;
use regex::Regex;
use crate::error::Error;
use crate::tilemap::Tilemap;
use crate::tileset::Tileset;

#[derive(Debug)]
#[derive(Default)]
pub struct Map {
    name: String,
    asm_filename: String,
    blk_filename: String,
    tileset: String,
    tileset_file: String,
    id: String,
    width: usize,
    height: usize,
    border_block: u8,
    border_block_tiles: Vec<u8>,
    blocks: Vec<u8>,
    tilemap: Tilemap,
}

impl Map {
    pub fn new(name: String, tileset: String) -> Self {
        let blk_filename = match name.as_str() {
            "Route32" => "Route32.blk".to_owned(),
            "Route40" => "Route40.blk".to_owned(),
            "Route36" => "Route36.blk".to_owned(),
            "Route44" => "Route44.blk".to_owned(),
            "Route28" => "Route28.blk".to_owned(),
            "BetaPlayersHouse2F" => "unused/BetaPlayersHouse2F.blk".to_owned(),
            "CeladonCity" => "CeladonCity.blk".to_owned(),
            "SaffronCity" => "SaffronCity.blk".to_owned(),
            "Route2" => "Route2.blk".to_owned(),
            "ElmsHouse" => "ElmsHouse.blk".to_owned(),
            "BetaSproutTower1" => "unused/BetaSproutTower1.blk".to_owned(),
            "Route11" => "Route11.blk".to_owned(),
            "BetaSproutTower5" => "unused/BetaSproutTower5.blk".to_owned(),
            "Route15" => "Route15.blk".to_owned(),
            "BetaSproutTower9" => "unused/BetaSproutTower9.blk".to_owned(),
            "Route19" => "Route19.blk".to_owned(),
            "BetaBlackthornCity" => "unused/BetaBlackthornCity.blk".to_owned(),
            "Route10South" => "Route10South.blk".to_owned(),
            "Pokecenter2F" => "Pokecenter2F.blk".to_owned(),
            "CinnabarPokecenter2FBeta" => "Pokecenter2F.blk".to_owned(),
            "CeruleanPokecenter2FBeta" => "Pokecenter2F.blk".to_owned(),
            "Route10Pokecenter2FBeta" => "Pokecenter2F.blk".to_owned(),
            "VermilionPokecenter2FBeta" => "Pokecenter2F.blk".to_owned(),
            "PewterPokecenter2FBeta" => "Pokecenter2F.blk".to_owned(),
            "FuchsiaPokecenter2FBeta" => "Pokecenter2F.blk".to_owned(),
            "LavenderPokecenter2FBeta" => "Pokecenter2F.blk".to_owned(),
            "CeladonPokecenter2FBeta" => "Pokecenter2F.blk".to_owned(),
            "ViridianPokecenter2FBeta" => "Pokecenter2F.blk".to_owned(),
            "SaffronPokecenter2FBeta" => "Pokecenter2F.blk".to_owned(),
            "Route41" => "Route41.blk".to_owned(),
            "Route33" => "Route33.blk".to_owned(),
            "Route45" => "Route45.blk".to_owned(),
            "Route29" => "Route29.blk".to_owned(),
            "Route37" => "Route37.blk".to_owned(),
            "LavenderTown" => "LavenderTown.blk".to_owned(),
            "PalletTown" => "PalletTown.blk".to_owned(),
            "Route25" => "Route25.blk".to_owned(),
            "Route24" => "Route24.blk".to_owned(),
            "BetaVioletCity" => "unused/BetaVioletCity.blk".to_owned(),
            "Route3" => "Route3.blk".to_owned(),
            "PewterCity" => "PewterCity.blk".to_owned(),
            "BetaSilverCaveOutside" => "unused/BetaSilverCaveOutside.blk".to_owned(),
            "BetaSproutTower2" => "unused/BetaSproutTower2.blk".to_owned(),
            "Route12" => "Route12.blk".to_owned(),
            "BetaGoldenrodCity" => "unused/BetaGoldenrodCity.blk".to_owned(),
            "Route20" => "Route20.blk".to_owned(),
            "BetaSproutTower6" => "unused/BetaSproutTower6.blk".to_owned(),
            "BetaPokecenter" => "unused/BetaPokecenter.blk".to_owned(),
            "Route30" => "Route30.blk".to_owned(),
            "Route26" => "Route26.blk".to_owned(),
            "Route42" => "Route42.blk".to_owned(),
            "Route34" => "Route34.blk".to_owned(),
            "Route46" => "Route46.blk".to_owned(),
            "FuchsiaCity" => "FuchsiaCity.blk".to_owned(),
            "Route38" => "Route38.blk".to_owned(),
            "BetaCianwoodCity" => "unused/BetaCianwoodCity.blk".to_owned(),
            "OlivineTimsHouse" => "House1.blk".to_owned(),
            "OlivineHouseBeta" => "House1.blk".to_owned(),
            "OlivinePunishmentSpeechHouse" => "House1.blk".to_owned(),
            "OlivineGoodRodHouse" => "House1.blk".to_owned(),
            "Route39Farmhouse" => "House1.blk".to_owned(),
            "MahoganyRedGyaradosSpeechHouse" => "House1.blk".to_owned(),
            "BlackthornDragonSpeechHouse" => "House1.blk".to_owned(),
            "BlackthornEmysHouse" => "House1.blk".to_owned(),
            "MoveDeletersHouse" => "House1.blk".to_owned(),
            "CeruleanGymBadgeSpeechHouse" => "House1.blk".to_owned(),
            "CeruleanPoliceStation" => "House1.blk".to_owned(),
            "CeruleanTradeSpeechHouse" => "House1.blk".to_owned(),
            "BillsHouse" => "House1.blk".to_owned(),
            "CharcoalKiln" => "House1.blk".to_owned(),
            "LakeOfRageHiddenPowerHouse" => "House1.blk".to_owned(),
            "LakeOfRageMagikarpHouse" => "House1.blk".to_owned(),
            "GoldenrodHappinessRater" => "House1.blk".to_owned(),
            "BillsFamilysHouse" => "House1.blk".to_owned(),
            "GoldenrodPPSpeechHouse" => "House1.blk".to_owned(),
            "GoldenrodNameRater" => "House1.blk".to_owned(),
            "VermilionFishingSpeechHouse" => "House1.blk".to_owned(),
            "VermilionMagnetTrainSpeechHouse" => "House1.blk".to_owned(),
            "VermilionDiglettsCaveSpeechHouse" => "House1.blk".to_owned(),
            "BluesHouse" => "House1.blk".to_owned(),
            "PewterNidoranSpeechHouse" => "House1.blk".to_owned(),
            "PewterSnoozeSpeechHouse" => "House1.blk".to_owned(),
            "BillsBrothersHouse" => "House1.blk".to_owned(),
            "LavenderSpeechHouse" => "House1.blk".to_owned(),
            "LavenderNameRater" => "House1.blk".to_owned(),
            "Route12SuperRodHouse" => "House1.blk".to_owned(),
            "Route28SteelWingHouse" => "House1.blk".to_owned(),
            "CeladonMansionRoofHouse" => "House1.blk".to_owned(),
            "Route16FuchsiaSpeechHouse" => "House1.blk".to_owned(),
            "ManiasHouse" => "House1.blk".to_owned(),
            "CianwoodPharmacy" => "House1.blk".to_owned(),
            "CianwoodPhotoStudio" => "House1.blk".to_owned(),
            "CianwoodLugiaSpeechHouse" => "House1.blk".to_owned(),
            "PokeSeersHouse" => "House1.blk".to_owned(),
            "ViridianNicknameSpeechHouse" => "House1.blk".to_owned(),
            "Route2NuggetHouse" => "House1.blk".to_owned(),
            "PlayersNeighborsHouse" => "House1.blk".to_owned(),
            "Route26HealHouse" => "House1.blk".to_owned(),
            "DayOfWeekSiblingsHouse" => "House1.blk".to_owned(),
            "Route27SandstormHouse" => "House1.blk".to_owned(),
            "MrPsychicsHouse" => "House1.blk".to_owned(),
            "Route5CleanseTagHouse" => "House1.blk".to_owned(),
            "CherrygroveGymSpeechHouse" => "House1.blk".to_owned(),
            "GuideGentsHouse" => "House1.blk".to_owned(),
            "CherrygroveEvolutionSpeechHouse" => "House1.blk".to_owned(),
            "Route30BerryHouse" => "House1.blk".to_owned(),
            "SafariZoneFuchsiaGateBeta" => "NorthSouthGate.blk".to_owned(),
            "Route19FuchsiaGate" => "NorthSouthGate.blk".to_owned(),
            "Route43MahoganyGate" => "NorthSouthGate.blk".to_owned(),
            "Route43Gate" => "NorthSouthGate.blk".to_owned(),
            "Route35GoldenrodGate" => "NorthSouthGate.blk".to_owned(),
            "Route36RuinsOfAlphGate" => "NorthSouthGate.blk".to_owned(),
            "Route34IlexForestGate" => "NorthSouthGate.blk".to_owned(),
            "Route6SaffronGate" => "NorthSouthGate.blk".to_owned(),
            "Route40BattleTowerGate" => "NorthSouthGate.blk".to_owned(),
            "Route2Gate" => "NorthSouthGate.blk".to_owned(),
            "Route29Route46Gate" => "NorthSouthGate.blk".to_owned(),
            "Route5SaffronGate" => "NorthSouthGate.blk".to_owned(),
            "BetaEcruteakCity" => "unused/BetaEcruteakCity.blk".to_owned(),
            "BetaCherrygroveCity" => "unused/BetaCherrygroveCity.blk".to_owned(),
            "CinnabarIsland" => "CinnabarIsland.blk".to_owned(),
            "Route4" => "Route4.blk".to_owned(),
            "Route8" => "Route8.blk".to_owned(),
            "BetaSproutTower3" => "unused/BetaSproutTower3.blk".to_owned(),
            "ViridianCity" => "ViridianCity.blk".to_owned(),
            "Route13" => "Route13.blk".to_owned(),
            "Route21" => "Route21.blk".to_owned(),
            "BetaSproutTower7" => "unused/BetaSproutTower7.blk".to_owned(),
            "Route17" => "Route17.blk".to_owned(),
            "BetaMahoganyTown" => "unused/BetaMahoganyTown.blk".to_owned(),
            "Route31" => "Route31.blk".to_owned(),
            "Route27" => "Route27.blk".to_owned(),
            "Route35" => "Route35.blk".to_owned(),
            "Route43" => "Route43.blk".to_owned(),
            "Route39" => "Route39.blk".to_owned(),
            "PlayersHouse1F" => "PlayersHouse1F.blk".to_owned(),
            "Route38EcruteakGate" => "EastWestGate.blk".to_owned(),
            "Route42EcruteakGate" => "EastWestGate.blk".to_owned(),
            "Route32RuinsOfAlphGate" => "EastWestGate.blk".to_owned(),
            "IlexForestAzaleaGate" => "EastWestGate.blk".to_owned(),
            "Route15FuchsiaGate" => "EastWestGate.blk".to_owned(),
            "Route8SaffronGate" => "EastWestGate.blk".to_owned(),
            "Route16Gate" => "EastWestGate.blk".to_owned(),
            "Route7SaffronGate" => "EastWestGate.blk".to_owned(),
            "Route17Route18Gate" => "EastWestGate.blk".to_owned(),
            "Route31VioletGate" => "EastWestGate.blk".to_owned(),
            "BetaAzaleaTown" => "unused/BetaAzaleaTown.blk".to_owned(),
            "VermilionCity" => "VermilionCity.blk".to_owned(),
            "BetaOlivineCity" => "unused/BetaOlivineCity.blk".to_owned(),
            "BetaNewBarkTown" => "unused/BetaNewBarkTown.blk".to_owned(),
            "ElmsLab" => "ElmsLab.blk".to_owned(),
            "CeruleanCity" => "CeruleanCity.blk".to_owned(),
            "Route1" => "Route1.blk".to_owned(),
            "Route5" => "Route5.blk".to_owned(),
            "Route9" => "Route9.blk".to_owned(),
            "Route22" => "Route22.blk".to_owned(),
            "Route14" => "Route14.blk".to_owned(),
            "BetaSproutTower8" => "unused/BetaSproutTower8.blk".to_owned(),
            "OlivineMart" => "Mart.blk".to_owned(),
            "EcruteakMart" => "Mart.blk".to_owned(),
            "BlackthornMart" => "Mart.blk".to_owned(),
            "CeruleanMart" => "Mart.blk".to_owned(),
            "AzaleaMart" => "Mart.blk".to_owned(),
            "VioletMart" => "Mart.blk".to_owned(),
            "VermilionMart" => "Mart.blk".to_owned(),
            "PewterMart" => "Mart.blk".to_owned(),
            "FuchsiaMart" => "Mart.blk".to_owned(),
            "LavenderMart" => "Mart.blk".to_owned(),
            "ViridianMart" => "Mart.blk".to_owned(),
            "SaffronMart" => "Mart.blk".to_owned(),
            "CherrygroveMart" => "Mart.blk".to_owned(),
            "Route10North" => "Route10North.blk".to_owned(),
            "BetaLakeOfRage" => "unused/BetaLakeOfRage.blk".to_owned(),
            "OlivinePokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "MahoganyPokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "EcruteakPokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "BlackthornPokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "CinnabarPokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "CeruleanPokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "Route10Pokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "AzaleaPokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "VioletPokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "Route32Pokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "GoldenrodPokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "VermilionPokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "PewterPokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "FuchsiaPokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "LavenderPokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "SilverCavePokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "CeladonPokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "CianwoodPokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "ViridianPokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "SaffronPokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "CherrygrovePokecenter1F" => "Pokecenter1F.blk".to_owned(),
            "BetaPewterMuseumOfScience1F" => "unused/BetaPewterMuseumOfScience1F.blk".to_owned(),
            "BetaPewterMuseumOfScience2F" => "unused/BetaPewterMuseumOfScience2F.blk".to_owned(),
            "EarlsPokemonAcademy" => "EarlsPokemonAcademy.blk".to_owned(),
            "BetaCinnabarPokemonLabHallway" => "unused/BetaCinnabarPokemonLabHallway.blk".to_owned(),
            "BetaCinnabarPokemonLabRoom1" => "unused/BetaCinnabarPokemonLabRoom1.blk".to_owned(),
            "BetaCinnabarPokemonLabRoom2" => "unused/BetaCinnabarPokemonLabRoom2.blk".to_owned(),
            "BetaCinnabarPokemonLabRoom3" => "unused/BetaCinnabarPokemonLabRoom3.blk".to_owned(),
            "GoldenrodDeptStore1F" => "DeptStore1F.blk".to_owned(),
            "CeladonDeptStore1F" => "DeptStore1F.blk".to_owned(),
            "GoldenrodDeptStore2F" => "DeptStore2F.blk".to_owned(),
            "CeladonDeptStore2F" => "DeptStore2F.blk".to_owned(),
            "GoldenrodDeptStore3F" => "DeptStore3F.blk".to_owned(),
            "CeladonDeptStore3F" => "DeptStore3F.blk".to_owned(),
            "GoldenrodDeptStore4F" => "DeptStore4F.blk".to_owned(),
            "CeladonDeptStore4F" => "DeptStore4F.blk".to_owned(),
            "GoldenrodDeptStore5F" => "DeptStore5F.blk".to_owned(),
            "CeladonDeptStore5F" => "DeptStore5F.blk".to_owned(),
            "GoldenrodDeptStore6F" => "DeptStore6F.blk".to_owned(),
            "CeladonDeptStore6F" => "DeptStore6F.blk".to_owned(),
            "GoldenrodDeptStoreElevator" => "DeptStoreElevator.blk".to_owned(),
            "CeladonDeptStoreElevator" => "DeptStoreElevator.blk".to_owned(),
            "CeladonMansion1F" => "CeladonMansion1F.blk".to_owned(),
            "CeladonMansion2F" => "CeladonMansion2F.blk".to_owned(),
            "CeladonMansion3F" => "CeladonMansion3F.blk".to_owned(),
            "CeladonMansionRoof" => "CeladonMansionRoof.blk".to_owned(),
            "BetaHouse" => "unused/BetaHouse.blk".to_owned(),
            "CeladonGameCorner" => "CeladonGameCorner.blk".to_owned(),
            "CeladonGameCornerPrizeRoom" => "CeladonGameCornerPrizeRoom.blk".to_owned(),
            "Colosseum" => "Colosseum.blk".to_owned(),
            "TradeCenter" => "TradeCenter.blk".to_owned(),
            "TimeCapsule" => "TradeCenter.blk".to_owned(),
            "EcruteakLugiaSpeechHouse" => "House2.blk".to_owned(),
            "EcruteakItemfinderHouse" => "House2.blk".to_owned(),
            "VioletNicknameSpeechHouse" => "House2.blk".to_owned(),
            "VioletKylesHouse" => "House2.blk".to_owned(),
            "BetaUnionCave" => "unused/BetaUnionCave.blk".to_owned(),
            "UnionCaveB1F" => "UnionCaveB1F.blk".to_owned(),
            "UnionCaveB2F" => "UnionCaveB2F.blk".to_owned(),
            "UnionCave1F" => "UnionCave1F.blk".to_owned(),
            "NationalPark" => "NationalPark.blk".to_owned(),
            "NationalParkBugContest" => "NationalPark.blk".to_owned(),
            "Route5UndergroundPathEntrance" => "UndergroundPathEntrance.blk".to_owned(),
            "Route6UndergroundPathEntrance" => "UndergroundPathEntrance.blk".to_owned(),
            "BetaCapsuleHouse" => "unused/BetaCapsuleHouse.blk".to_owned(),
            "KurtsHouse" => "KurtsHouse.blk".to_owned(),
            "GoldenrodMagnetTrainStation" => "GoldenrodMagnetTrainStation.blk".to_owned(),
            "RuinsOfAlphOutside" => "RuinsOfAlphOutside.blk".to_owned(),
            "BetaRuinsOfAlphUnsolvedPuzzleRoom" => "unused/BetaRuinsOfAlphUnsolvedPuzzleRoom.blk".to_owned(),
            "RuinsOfAlphInnerChamber" => "RuinsOfAlphInnerChamber.blk".to_owned(),
            "RuinsOfAlphHoOhChamber" => "RuinsOfAlphPuzzleChamber.blk".to_owned(),
            "RuinsOfAlphKabutoChamber" => "RuinsOfAlphPuzzleChamber.blk".to_owned(),
            "RuinsOfAlphOmanyteChamber" => "RuinsOfAlphPuzzleChamber.blk".to_owned(),
            "RuinsOfAlphAerodactylChamber" => "RuinsOfAlphPuzzleChamber.blk".to_owned(),
            "SproutTower1F" => "SproutTower1F.blk".to_owned(),
            "BetaSproutTowerCutOut1" => "unused/BetaSproutTowerCutOut1.blk".to_owned(),
            "SproutTower2F" => "SproutTower2F.blk".to_owned(),
            "BetaSproutTowerCutOut2" => "unused/BetaSproutTowerCutOut2.blk".to_owned(),
            "SproutTower3F" => "SproutTower3F.blk".to_owned(),
            "BetaSproutTowerCutOut3" => "unused/BetaSproutTowerCutOut3.blk".to_owned(),
            "RadioTower1F" => "RadioTower1F.blk".to_owned(),
            "RadioTower2F" => "RadioTower2F.blk".to_owned(),
            "RadioTower3F" => "RadioTower3F.blk".to_owned(),
            "RadioTower4F" => "RadioTower4F.blk".to_owned(),
            "RadioTower5F" => "RadioTower5F.blk".to_owned(),
            "NewBarkTown" => "NewBarkTown.blk".to_owned(),
            "CherrygroveCity" => "CherrygroveCity.blk".to_owned(),
            "VioletCity" => "VioletCity.blk".to_owned(),
            "AzaleaTown" => "AzaleaTown.blk".to_owned(),
            "CianwoodCity" => "CianwoodCity.blk".to_owned(),
            "GoldenrodCity" => "GoldenrodCity.blk".to_owned(),
            "OlivineCity" => "OlivineCity.blk".to_owned(),
            "EcruteakCity" => "EcruteakCity.blk".to_owned(),
            "MahoganyTown" => "MahoganyTown.blk".to_owned(),
            "LakeOfRage" => "LakeOfRage.blk".to_owned(),
            "BlackthornCity" => "BlackthornCity.blk".to_owned(),
            "SilverCaveOutside" => "SilverCaveOutside.blk".to_owned(),
            "Route6" => "Route6.blk".to_owned(),
            "Route7" => "Route7.blk".to_owned(),
            "Route16" => "Route16.blk".to_owned(),
            "Route18" => "Route18.blk".to_owned(),
            "GoldenrodUnderground" => "GoldenrodUnderground.blk".to_owned(),
            "GoldenrodUndergroundSwitchRoomEntrances" => "GoldenrodUndergroundSwitchRoomEntrances.blk".to_owned(),
            "GoldenrodDeptStoreB1F" => "GoldenrodDeptStoreB1F.blk".to_owned(),
            "GoldenrodUndergroundWarehouse" => "GoldenrodUndergroundWarehouse.blk".to_owned(),
            "BetaElevator" => "unused/BetaElevator.blk".to_owned(),
            "TinTower1F" => "TinTower1F.blk".to_owned(),
            "TinTower2F" => "TinTower2F.blk".to_owned(),
            "TinTower3F" => "TinTower3F.blk".to_owned(),
            "TinTower4F" => "TinTower4F.blk".to_owned(),
            "TinTower5F" => "TinTower5F.blk".to_owned(),
            "TinTower6F" => "TinTower6F.blk".to_owned(),
            "TinTower7F" => "TinTower7F.blk".to_owned(),
            "TinTower8F" => "TinTower8F.blk".to_owned(),
            "TinTower9F" => "TinTower9F.blk".to_owned(),
            "TinTowerRoof" => "TinTowerRoof.blk".to_owned(),
            "BurnedTower1F" => "BurnedTower1F.blk".to_owned(),
            "BurnedTowerB1F" => "BurnedTowerB1F.blk".to_owned(),
            "BetaCaveTestMap" => "unused/BetaCaveTestMap.blk".to_owned(),
            "MountMortar1FOutside" => "MountMortar1FOutside.blk".to_owned(),
            "MountMortar1FInside" => "MountMortar1FInside.blk".to_owned(),
            "MountMortar2FInside" => "MountMortar2FInside.blk".to_owned(),
            "MountMortarB1F" => "MountMortarB1F.blk".to_owned(),
            "IcePath1F" => "IcePath1F.blk".to_owned(),
            "IcePathB1F" => "IcePathB1F.blk".to_owned(),
            "IcePathB2FMahoganySide" => "IcePathB2FMahoganySide.blk".to_owned(),
            "IcePathB2FBlackthornSide" => "IcePathB2FBlackthornSide.blk".to_owned(),
            "IcePathB3F" => "IcePathB3F.blk".to_owned(),
            "WhirlIslandNW" => "WhirlIslandNW.blk".to_owned(),
            "WhirlIslandNE" => "WhirlIslandNE.blk".to_owned(),
            "WhirlIslandSW" => "WhirlIslandSW.blk".to_owned(),
            "WhirlIslandCave" => "WhirlIslandCave.blk".to_owned(),
            "WhirlIslandSE" => "WhirlIslandSE.blk".to_owned(),
            "WhirlIslandB1F" => "WhirlIslandB1F.blk".to_owned(),
            "WhirlIslandB2F" => "WhirlIslandB2F.blk".to_owned(),
            "WhirlIslandLugiaChamber" => "WhirlIslandLugiaChamber.blk".to_owned(),
            "SilverCaveRoom1" => "SilverCaveRoom1.blk".to_owned(),
            "SilverCaveRoom2" => "SilverCaveRoom2.blk".to_owned(),
            "SilverCaveRoom3" => "SilverCaveRoom3.blk".to_owned(),
            "BetaRocketHideoutB2F" => "unused/BetaRocketHideoutB2F.blk".to_owned(),
            "BetaRocketHideoutB1F" => "unused/BetaRocketHideoutB1F.blk".to_owned(),
            "BetaRocketHideout1F" => "unused/BetaRocketHideout1F.blk".to_owned(),
            "BetaRocketHideoutB3F" => "unused/BetaRocketHideoutB3F.blk".to_owned(),
            "MahoganyMart1F" => "GiftShop.blk".to_owned(),
            "MountMoonGiftShop" => "GiftShop.blk".to_owned(),
            "TeamRocketBaseB1F" => "TeamRocketBaseB1F.blk".to_owned(),
            "TeamRocketBaseB2F" => "TeamRocketBaseB2F.blk".to_owned(),
            "TeamRocketBaseB3F" => "TeamRocketBaseB3F.blk".to_owned(),
            "BetaRoute23" => "unused/BetaRoute23.blk".to_owned(),
            "IndigoPlateauPokecenter1F" => "IndigoPlateauPokecenter1F.blk".to_owned(),
            "WillsRoom" => "WillsRoom.blk".to_owned(),
            "KogasRoom" => "KogasRoom.blk".to_owned(),
            "BrunosRoom" => "BrunosRoom.blk".to_owned(),
            "KarensRoom" => "KarensRoom.blk".to_owned(),
            "AzaleaGym" => "AzaleaGym.blk".to_owned(),
            "VioletGym" => "VioletGym.blk".to_owned(),
            "GoldenrodGym" => "GoldenrodGym.blk".to_owned(),
            "EcruteakGym" => "EcruteakGym.blk".to_owned(),
            "MahoganyGym" => "MahoganyGym.blk".to_owned(),
            "OlivineGym" => "OlivineGym.blk".to_owned(),
            "BetaUnknownGym" => "unused/BetaUnknownGym.blk".to_owned(),
            "CianwoodGym" => "CianwoodGym.blk".to_owned(),
            "BlackthornGym1F" => "BlackthornGym1F.blk".to_owned(),
            "BlackthornGym2F" => "BlackthornGym2F.blk".to_owned(),
            "OlivineLighthouse1F" => "OlivineLighthouse1F.blk".to_owned(),
            "OlivineLighthouse2F" => "OlivineLighthouse2F.blk".to_owned(),
            "OlivineLighthouse3F" => "OlivineLighthouse3F.blk".to_owned(),
            "OlivineLighthouse4F" => "OlivineLighthouse4F.blk".to_owned(),
            "OlivineLighthouse5F" => "OlivineLighthouse5F.blk".to_owned(),
            "OlivineLighthouse6F" => "OlivineLighthouse6F.blk".to_owned(),
            "BetaSlowpokeWell1F" => "unused/BetaSlowpokeWell1F.blk".to_owned(),
            "SlowpokeWellB1F" => "SlowpokeWellB1F.blk".to_owned(),
            "SlowpokeWellB2F" => "SlowpokeWellB2F.blk".to_owned(),
            "IlexForest" => "IlexForest.blk".to_owned(),
            "DarkCaveVioletEntrance" => "DarkCaveVioletEntrance.blk".to_owned(),
            "DarkCaveBlackthornEntrance" => "DarkCaveBlackthornEntrance.blk".to_owned(),
            "RuinsOfAlphResearchCenter" => "RuinsOfAlphResearchCenter.blk".to_owned(),
            "GoldenrodBikeShop" => "GoldenrodBikeShop.blk".to_owned(),
            "DanceTheater" => "DanceTheater.blk".to_owned(),
            "EcruteakTinTowerEntrance" => "EcruteakTinTowerEntrance.blk".to_owned(),
            "GoldenrodGameCorner" => "GoldenrodGameCorner.blk".to_owned(),
            "Route35NationalParkGate" => "Route35NationalParkGate.blk".to_owned(),
            "Route36NationalParkGate" => "Route36NationalParkGate.blk".to_owned(),
            "FastShip1F" => "FastShip1F.blk".to_owned(),
            "FastShipB1F" => "FastShipB1F.blk".to_owned(),
            "BetaFastShipInsideCutOut" => "unused/BetaFastShipInsideCutOut.blk".to_owned(),
            "FastShipCabins_NNW_NNE_NE" => "FastShipCabins_NNW_NNE_NE.blk".to_owned(),
            "FastShipCabins_SW_SSW_NW" => "FastShipCabins_SW_SSW_NW.blk".to_owned(),
            "FastShipCabins_SE_SSE_CaptainsCabin" => "FastShipCabins_SE_SSE_CaptainsCabin.blk".to_owned(),
            "OlivinePort" => "OlivinePort.blk".to_owned(),
            "VermilionPort" => "VermilionPort.blk".to_owned(),
            "OlivineCafe" => "OlivineCafe.blk".to_owned(),
            "SafariZoneMainOffice" => "OlivineCafe.blk".to_owned(),
            "PlayersHouse2F" => "PlayersHouse2F.blk".to_owned(),
            "SaffronMagnetTrainStation" => "SaffronMagnetTrainStation.blk".to_owned(),
            "CeruleanGym" => "CeruleanGym.blk".to_owned(),
            "VermilionGym" => "VermilionGym.blk".to_owned(),
            "SaffronGym" => "SaffronGym.blk".to_owned(),
            "PowerPlant" => "PowerPlant.blk".to_owned(),
            "PokemonFanClub" => "PokemonFanClub.blk".to_owned(),
            "SafariZoneWardensHome" => "PokemonFanClub.blk".to_owned(),
            "FightingDojo" => "FightingDojo.blk".to_owned(),
            "SilphCo1F" => "SilphCo1F.blk".to_owned(),
            "ViridianGym" => "ViridianGym.blk".to_owned(),
            "TrainerHouse1F" => "TrainerHouse1F.blk".to_owned(),
            "TrainerHouseB1F" => "TrainerHouseB1F.blk".to_owned(),
            "RedsHouse1F" => "RedsHouse1F.blk".to_owned(),
            "RedsHouse2F" => "RedsHouse2F.blk".to_owned(),
            "OaksLab" => "OaksLab.blk".to_owned(),
            "MrFujisHouse" => "MrFujisHouse.blk".to_owned(),
            "LavRadioTower1F" => "LavRadioTower1F.blk".to_owned(),
            "SilverCaveItemRooms" => "SilverCaveItemRooms.blk".to_owned(),
            "DayCare" => "DayCare.blk".to_owned(),
            "SoulHouse" => "SoulHouse.blk".to_owned(),
            "PewterGym" => "PewterGym.blk".to_owned(),
            "CeladonGym" => "CeladonGym.blk".to_owned(),
            "BetaCeladonMansion1F" => "unused/BetaCeladonMansion1F.blk".to_owned(),
            "CeladonCafe" => "CeladonCafe.blk".to_owned(),
            "BetaCeladonMansion2F" => "unused/BetaCeladonMansion2F.blk".to_owned(),
            "RockTunnel1F" => "RockTunnel1F.blk".to_owned(),
            "RockTunnelB1F" => "RockTunnelB1F.blk".to_owned(),
            "DiglettsCave" => "DiglettsCave.blk".to_owned(),
            "MountMoon" => "MountMoon.blk".to_owned(),
            "SeafoamGym" => "SeafoamGym.blk".to_owned(),
            "MrPokemonsHouse" => "MrPokemonsHouse.blk".to_owned(),
            "VictoryRoadGate" => "VictoryRoadGate.blk".to_owned(),
            "OlivinePortPassage" => "PortPassage.blk".to_owned(),
            "VermilionPortPassage" => "PortPassage.blk".to_owned(),
            "FuchsiaGym" => "FuchsiaGym.blk".to_owned(),
            "SafariZoneBeta" => "SafariZoneBeta.blk".to_owned(),
            "UndergroundPath" => "UndergroundPath.blk".to_owned(),
            "Route39Barn" => "Route39Barn.blk".to_owned(),
            "VictoryRoad" => "VictoryRoad.blk".to_owned(),
            "Route23" => "Route23.blk".to_owned(),
            "LancesRoom" => "LancesRoom.blk".to_owned(),
            "HallOfFame" => "HallOfFame.blk".to_owned(),
            "CopycatsHouse1F" => "CopycatsHouse1F.blk".to_owned(),
            "CopycatsHouse2F" => "CopycatsHouse2F.blk".to_owned(),
            "GoldenrodFlowerShop" => "GoldenrodFlowerShop.blk".to_owned(),
            "MountMoonSquare" => "MountMoonSquare.blk".to_owned(),
            "WiseTriosRoom" => "WiseTriosRoom.blk".to_owned(),
            "DragonsDen1F" => "DragonsDen1F.blk".to_owned(),
            "DragonsDenB1F" => "DragonsDenB1F.blk".to_owned(),
            "TohjoFalls" => "TohjoFalls.blk".to_owned(),
            "RuinsOfAlphHoOhItemRoom" => "RuinsOfAlphItemRoom.blk".to_owned(),
            "RuinsOfAlphKabutoItemRoom" => "RuinsOfAlphItemRoom.blk".to_owned(),
            "RuinsOfAlphOmanyteItemRoom" => "RuinsOfAlphItemRoom.blk".to_owned(),
            "RuinsOfAlphAerodactylItemRoom" => "RuinsOfAlphItemRoom.blk".to_owned(),
            "RuinsOfAlphHoOhWordRoom" => "RuinsOfAlphHoOhWordRoom.blk".to_owned(),
            "RuinsOfAlphKabutoWordRoom" => "RuinsOfAlphKabutoWordRoom.blk".to_owned(),
            "RuinsOfAlphOmanyteWordRoom" => "RuinsOfAlphOmanyteWordRoom.blk".to_owned(),
            "RuinsOfAlphAerodactylWordRoom" => "RuinsOfAlphAerodactylWordRoom.blk".to_owned(),
            "DragonShrine" => "DragonShrine.blk".to_owned(),
            "BattleTower1F" => "BattleTower1F.blk".to_owned(),
            "BattleTowerBattleRoom" => "BattleTowerBattleRoom.blk".to_owned(),
            "PokecomCenterAdminOfficeMobile" => "PokecomCenterAdminOfficeMobile.blk".to_owned(),
            "MobileTradeRoom" => "MobileTradeRoom.blk".to_owned(),
            "MobileBattleRoom" => "MobileBattleRoom.blk".to_owned(),
            "BattleTowerHallway" => "BattleTowerHallway.blk".to_owned(),
            "BattleTowerElevator" => "BattleTowerElevator.blk".to_owned(),
            "BattleTowerOutside" => "BattleTowerOutside.blk".to_owned(),
            "BetaBlank" => "unused/BetaBlank.blk".to_owned(),
            "GoldenrodDeptStoreRoof" => "GoldenrodDeptStoreRoof.blk".to_owned(),
            _ => panic!()
        };

        Self {
            name: name.clone(),
            asm_filename: format!("{}.asm", name.clone()),
            blk_filename,
            tileset: tileset.clone(),
            tileset_file: format!("{}_metatiles.bin", tileset.to_lowercase().drain(8..).collect::<String>()),
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn asm_filename(&self) -> &str {
        &self.asm_filename
    }

    pub fn tileset(&self) -> &str {
        &self.tileset
    }

    pub fn tileset_file(&self) -> &str {
        &self.tileset_file
    }

    pub fn width(&self) -> &usize {
        &self.width
    }

    pub fn height(&self) -> &usize {
        &self.height
    }

    pub fn tilemap(&self) -> &Tilemap {
        &self.tilemap
    }

    pub fn load_blocks(&mut self, dir: &str) -> Result<(), Error> {
        let path = Path::new(dir).join(self.blk_filename.clone());
        let file_contents = fs::read(path).map_err(Error::IoError)?;
        self.blocks = file_contents.into_iter().collect();

        Ok(())
    }

    pub fn load_attributes(&mut self, file: &str) -> Result<(), Error> {
        let file_contents = fs::read_to_string(file).map_err(Error::IoError)?;
        let re = Regex::new(format!(r"(?m)map_attributes\s*{},\s*(.*?),\s*\$(.*?),.*", self.name).as_str()).map_err(Error::RegexError)?;

        for line in file_contents.lines() {
            let captures = re.captures(line);

            if let Some(captures) = captures {
                self.id = captures[1].to_owned();
                self.border_block = u8::from_str_radix(&captures[2], 16).map_err(|_| Error::MapBorderBlockConversionFailed)?;

                return Ok(());
            }
        }

        Err(Error::MapAttributesNotFound)
    }

    pub fn load_size(&mut self, file: &str) -> Result<(), Error> {
        let file_contents = fs::read_to_string(file).map_err(Error::IoError)?;
        let re = Regex::new(format!(r"(?m)map_const\s*{},\s*(\d+),\s*(\d+)", self.id).as_str()).map_err(Error::RegexError)?;

        for line in file_contents.lines() {
            let captures = re.captures(line);

            if let Some(captures) = captures {
                self.width = captures[1].to_owned().parse().map_err(|_| Error::MapSizeConversionFailed)?;
                self.height = captures[2].to_owned().parse().map_err(|_| Error::MapSizeConversionFailed)?;

                return Ok(());
            }
        }

        Err(Error::MapConstNotFound)
    }

    pub fn build_tilemap(&mut self, tileset: &Tileset) -> Result<(), Error> {
        self.border_block_tiles.resize(16, 0);
        self.border_block_tiles.copy_from_slice(tileset.blocks().get(&self.border_block).ok_or(Error::BlockNotFound)?);

        self.tilemap.build_tilemap(self.width, &self.blocks, &tileset.blocks())
    }

    pub fn get_tilemap_at_player_pos(&self, x: isize, y: isize) -> Result<[u8; 360], Error> {
        let tile_x = x * 2;
        let tile_y = y * 2;
        let offset_x = tile_x % 4;
        let offset_y = tile_y % 4;
        let top_left = (tile_x - 8, tile_y - 8);
        let bottom_right = (tile_x + 12, tile_y + 10);

        let mut window_tilemap = Tilemap::default();
        window_tilemap.build_tilemap(6, &Vec::from([self.border_block; 36]), &HashMap::from([(self.border_block, self.border_block_tiles.clone())]))?;

        for current_x in cmp::max(top_left.0, 0)..=bottom_right.0 {
            for current_y in cmp::max(top_left.1, 0)..=bottom_right.1 {
                let tile = self.tilemap.tilemap().get(&(current_x.try_into().unwrap(), current_y.try_into().unwrap()));

                if let Some(tile) = tile {
                    window_tilemap.set_tile_at((current_x - top_left.0 + offset_x).try_into().unwrap(), (current_y - top_left.1 + offset_y).try_into().unwrap(), *tile);
                }
            }
        }

        let real_tilemap: HashMap<(usize, usize), u8> = window_tilemap.tilemap()
            .iter()
            .filter(|(&coords, &tile)| coords.0 >= offset_x as usize && coords.0 < (20 + offset_x) as usize && coords.1 >= offset_y as usize && coords.1 < (18 + offset_y) as usize)
            .map(|(coords, tile)| ((coords.0 - offset_x as usize, coords.1 - offset_y as usize), tile.clone()))
            .collect();

        window_tilemap.set_tilemap(real_tilemap);

        let mut res: [u8; 360] = [0; 360];
        let mut i = 0;

        for y in 0..window_tilemap.height() {
            for x in 0..window_tilemap.width() {
                let tile = window_tilemap.tilemap().get(&(x, y)).unwrap();
                res[i] = *tile;

                i += 1;
            }
        }

        Ok(res)
    }
}