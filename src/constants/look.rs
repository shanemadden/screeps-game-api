use crate::{constants::Terrain, enums::StructureObject, objects::*};
use enum_iterator::IntoEnumIterator;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsCast};

/// Translates `LOOK_*` constants.
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoEnumIterator)]
pub enum Look {
    Creeps = "creep",
    Energy = "energy",
    Resources = "resource",
    Sources = "source",
    Minerals = "mineral",
    Structures = "structure",
    Flags = "flag",
    ConstructionSites = "constructionSite",
    Nukes = "nuke",
    Terrain = "terrain",
    Tombstones = "tombstone",
    PowerCreeps = "powerCreep",
    Deposits = "deposit",
    Ruins = "ruin",
    // todo these seem to not work when conditionally compiled out - they're not hurting to leave
    // in but need to figure that out
    //#[cfg(feature = "score")]
    ScoreContainers = "scoreContainer",
    //#[cfg(feature = "score")]
    ScoreCollectors = "scoreCollector",
    //#[cfg(feature = "symbols")]
    SymbolContainers = "symbolContainer",
    //#[cfg(feature = "symbols")]
    SymbolDecoders = "symbolDecoder",
}

//TODO: wiarchbe: Add back in calculated doc.
macro_rules! typesafe_look_constants {
    (
        $(
            $vis:vis struct $constant_name:ident = ($value:expr, $result:path, $conversion_method:expr);
        )*
    ) => (
        $(
            #[allow(bad_style)]
            $vis struct $constant_name;
            impl LookConstant for $constant_name {
                type Item = $result;

                fn convert_and_check_item(reference: JsValue) -> Self::Item {
                    $conversion_method(reference)
                }

                #[inline]
                fn look_code() -> Look {
                    $value
                }
            }
        )*
    );
}

pub trait LookConstant {
    type Item;

    fn convert_and_check_item(reference: JsValue) -> Self::Item;

    fn look_code() -> Look;
}

typesafe_look_constants! {
    pub struct CREEPS = (Look::Creeps, Creep, Into::into);
    pub struct ENERGY = (Look::Energy, Resource, Into::into);
    pub struct RESOURCES = (Look::Resources, Resource, Into::into);
    pub struct SOURCES = (Look::Sources, Source, Into::into);
    pub struct MINERALS = (Look::Minerals, Mineral, Into::into);
    pub struct DEPOSITS = (Look::Deposits, Deposit, Into::into);
    pub struct STRUCTURES = (Look::Structures, StructureObject, Into::into);
    pub struct FLAGS = (Look::Flags, Flag, Into::into);
    pub struct CONSTRUCTION_SITES = (Look::ConstructionSites, ConstructionSite,
        Into::into);
    pub struct NUKES = (Look::Nukes, Nuke, Into::into);
    pub struct TERRAIN = (Look::Terrain, Terrain, Terrain::from_look_constant_jsvalue);
    pub struct TOMBSTONES = (Look::Tombstones, Tombstone, Into::into);
    pub struct POWER_CREEPS = (Look::PowerCreeps, PowerCreep, Into::into);
    pub struct RUINS = (Look::Ruins, Ruin, Into::into);
}

#[cfg(feature = "score")]
typesafe_look_constants! {
    pub struct SCORE_CONTAINERS = (Look::ScoreContainers, ScoreContainer, Into::into);
    pub struct SCORE_COLLECTORS = (Look::ScoreCollectors, ScoreCollector, Into::into);
}

#[cfg(feature = "symbols")]
typesafe_look_constants! {
    pub struct SYMBOL_CONTAINERS = (Look::SymbolContainers, SymbolContainer, Into::into);
    pub struct SYMBOL_DECODERS = (Look::SymbolDecoders, SymbolDecoder, Into::into);
}

#[derive(Debug)]
pub enum LookResult {
    Creep(Creep),
    Energy(Resource),
    Resource(Resource),
    Source(Source),
    Mineral(Mineral),
    Deposit(Deposit),
    Structure(Structure),
    Flag(Flag),
    ConstructionSite(ConstructionSite),
    Nuke(Nuke),
    Terrain(Terrain),
    Tombstone(Tombstone),
    PowerCreep(PowerCreep),
    Ruin(Ruin),
    #[cfg(feature = "score")]
    ScoreContainer(ScoreContainer),
    #[cfg(feature = "score")]
    ScoreCollector(ScoreCollector),
    #[cfg(feature = "symbols")]
    SymbolContainer(SymbolContainer),
    #[cfg(feature = "symbols")]
    SymbolDecoder(SymbolDecoder),
}

impl LookResult {
    pub(crate) fn from_result_with_type(result: JsLookResult, t: Look) -> Self {
        match t {
            Look::Creeps => Self::Creep(result.creep()),
            Look::Energy => Self::Energy(result.energy()),
            Look::Resources => Self::Resource(result.resource()),
            Look::Sources => Self::Source(result.source()),
            Look::Minerals => Self::Mineral(result.mineral()),
            Look::Deposits => Self::Deposit(result.deposit()),
            Look::Structures => Self::Structure(result.structure()),
            Look::Flags => Self::Flag(result.flag()),
            Look::ConstructionSites => Self::ConstructionSite(result.construction_site()),
            Look::Nukes => Self::Nuke(result.nuke()),
            Look::Terrain => Self::Terrain(Terrain::from_look_constant_str(&result.terrain())),
            Look::Tombstones => Self::Tombstone(result.tombstone()),
            Look::PowerCreeps => Self::PowerCreep(result.power_creep()),
            Look::Ruins => Self::Ruin(result.ruin()),
            #[cfg(feature = "score")]
            Look::ScoreContainers => Self::ScoreContainer(result.score_container()),
            #[cfg(feature = "score")]
            Look::ScoreCollectors => Self::ScoreCollector(result.score_collector()),
            #[cfg(feature = "symbols")]
            Look::SymbolContainers => Self::SymbolContainer(result.symbol_container()),
            #[cfg(feature = "symbols")]
            Look::SymbolDecoders => Self::SymbolDecoder(result.symbol_decoder()),
            _ => panic!("look result type not matched, object type feature may be disabled?"),
        }
    }

    pub(crate) fn from_jsvalue_unknown_type(v: JsValue) -> Self {
        let result: JsLookResult = v.unchecked_into();
        let rt = result.result_type();
        Self::from_result_with_type(result, rt)
    }
}

#[derive(Debug)]
pub struct PositionedLookResult {
    pub x: u8,
    pub y: u8,
    pub look_result: LookResult,
}

impl PositionedLookResult {
    pub(crate) fn from_jsvalue_with_type(v: JsValue, t: Look) -> Self {
        let result: JsLookResult = v.unchecked_into();
        let x = result.x();
        let y = result.y();
        let look_result = LookResult::from_result_with_type(result, t);
        Self { x, y, look_result }
    }

    pub(crate) fn from_jsvalue_unknown_type(v: JsValue) -> Self {
        let result: JsLookResult = v.unchecked_into();
        let rt = result.result_type();
        let x = result.x();
        let y = result.y();
        let look_result = LookResult::from_result_with_type(result, rt);
        Self { x, y, look_result }
    }
}

// internal accessors for results for look functions, any of which may be
// undefined in different kinds of look return calls
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub(crate) type JsLookResult;
    #[wasm_bindgen(method, getter = type)]
    fn result_type(this: &JsLookResult) -> Look;
    #[wasm_bindgen(method, getter)]
    fn x(this: &JsLookResult) -> u8;
    #[wasm_bindgen(method, getter)]
    fn y(this: &JsLookResult) -> u8;
    #[wasm_bindgen(method, getter)]
    fn creep(this: &JsLookResult) -> Creep;
    #[wasm_bindgen(method, getter)]
    fn energy(this: &JsLookResult) -> Resource;
    #[wasm_bindgen(method, getter)]
    fn resource(this: &JsLookResult) -> Resource;
    #[wasm_bindgen(method, getter)]
    fn source(this: &JsLookResult) -> Source;
    #[wasm_bindgen(method, getter)]
    fn mineral(this: &JsLookResult) -> Mineral;
    #[wasm_bindgen(method, getter)]
    fn deposit(this: &JsLookResult) -> Deposit;
    #[wasm_bindgen(method, getter)]
    fn structure(this: &JsLookResult) -> Structure;
    #[wasm_bindgen(method, getter)]
    fn flag(this: &JsLookResult) -> Flag;
    #[wasm_bindgen(method, getter = constructionSite)]
    fn construction_site(this: &JsLookResult) -> ConstructionSite;
    #[wasm_bindgen(method, getter)]
    fn nuke(this: &JsLookResult) -> Nuke;
    // note that this one is a string representing a terrain constant, and must be
    // converted
    #[wasm_bindgen(method, getter)]
    fn terrain(this: &JsLookResult) -> String;
    #[wasm_bindgen(method, getter)]
    fn tombstone(this: &JsLookResult) -> Tombstone;
    #[wasm_bindgen(method, getter = powerCreep)]
    fn power_creep(this: &JsLookResult) -> PowerCreep;
    #[wasm_bindgen(method, getter)]
    fn ruin(this: &JsLookResult) -> Ruin;
    #[cfg(feature = "score")]
    #[wasm_bindgen(method, getter = scoreContainer)]
    fn score_container(this: &JsLookResult) -> ScoreContainer;
    #[cfg(feature = "score")]
    #[wasm_bindgen(method, getter = scoreCollector)]
    fn score_collector(this: &JsLookResult) -> ScoreCollector;
    #[cfg(feature = "symbols")]
    #[wasm_bindgen(method, getter = symbolContainer)]
    fn symbol_container(this: &JsLookResult) -> SymbolContainer;
    #[cfg(feature = "symbols")]
    #[wasm_bindgen(method, getter = symbolDecoder)]
    fn symbol_decoder(this: &JsLookResult) -> SymbolDecoder;
}
