use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum CharacterName {
    Albedo,
    Aloy,
    Amber,
    AratakiItto,
    Barbara,
    Beidou,
    Bennett,
    Chongyun,
    Diluc,
    Diona,
    Eula,
    Fischl,
    Ganyu,
    Gorou,
    HuTao,
    Jean,
    KaedeharaKazuha,
    Kaeya,
    KamisatoAyaka,
    Keqing,
    Klee,
    KujouSara,
    Lisa,
    Mona,
    Ningguang,
    Noelle,
    Qiqi,
    RaidenShogun,
    Razor,
    Rosaria,
    SangonomiyaKokomi,
    Sayu,
    Sucrose,
    Tartaglia,
    Thoma,
    Traveler,
    Venti,
    Xiangling,
    Xiao,
    Xingqiu,
    Xinyan,
    Yanfei,
    Yoimiya,
    Zhongli,
}