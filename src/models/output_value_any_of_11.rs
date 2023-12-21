/*
 * Attio API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@attio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputValueAnyOf11 {
    /// The raw, original phone number, as inputted.
    #[serde(rename = "original_phone_number", deserialize_with = "Option::deserialize")]
    pub original_phone_number: Option<serde_json::Value>,
    /// The ISO 3166-1 alpha-2 country code representing the country that this phone number belongs to.
    #[serde(rename = "country_code", deserialize_with = "Option::deserialize")]
    pub country_code: Option<CountryCode>,
    #[serde(rename = "phone_number", deserialize_with = "Option::deserialize")]
    pub phone_number: Option<serde_json::Value>,
    /// The attribute type of the value.
    #[serde(rename = "attribute_type", deserialize_with = "Option::deserialize")]
    pub attribute_type: Option<AttributeType>,
}

impl OutputValueAnyOf11 {
    pub fn new(original_phone_number: Option<serde_json::Value>, country_code: Option<CountryCode>, phone_number: Option<serde_json::Value>, attribute_type: Option<AttributeType>) -> OutputValueAnyOf11 {
        OutputValueAnyOf11 {
            original_phone_number,
            country_code,
            phone_number,
            attribute_type,
        }
    }
}

/// The ISO 3166-1 alpha-2 country code representing the country that this phone number belongs to.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CountryCode {
    #[serde(rename = "AF")]
    Af,
    #[serde(rename = "AX")]
    Ax,
    #[serde(rename = "AL")]
    Al,
    #[serde(rename = "DZ")]
    Dz,
    #[serde(rename = "AS")]
    As,
    #[serde(rename = "AD")]
    Ad,
    #[serde(rename = "AO")]
    Ao,
    #[serde(rename = "AI")]
    Ai,
    #[serde(rename = "AQ")]
    Aq,
    #[serde(rename = "AG")]
    Ag,
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "AM")]
    Am,
    #[serde(rename = "AW")]
    Aw,
    #[serde(rename = "AU")]
    Au,
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "AZ")]
    Az,
    #[serde(rename = "BS")]
    Bs,
    #[serde(rename = "BH")]
    Bh,
    #[serde(rename = "BD")]
    Bd,
    #[serde(rename = "BB")]
    Bb,
    #[serde(rename = "BY")]
    By,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "BZ")]
    Bz,
    #[serde(rename = "BJ")]
    Bj,
    #[serde(rename = "BM")]
    Bm,
    #[serde(rename = "BT")]
    Bt,
    #[serde(rename = "BO")]
    Bo,
    #[serde(rename = "BA")]
    Ba,
    #[serde(rename = "BW")]
    Bw,
    #[serde(rename = "BV")]
    Bv,
    #[serde(rename = "BR")]
    Br,
    #[serde(rename = "IO")]
    Io,
    #[serde(rename = "BN")]
    Bn,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "BF")]
    Bf,
    #[serde(rename = "BI")]
    Bi,
    #[serde(rename = "KH")]
    Kh,
    #[serde(rename = "CM")]
    Cm,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "CV")]
    Cv,
    #[serde(rename = "KY")]
    Ky,
    #[serde(rename = "CF")]
    Cf,
    #[serde(rename = "TD")]
    Td,
    #[serde(rename = "CL")]
    Cl,
    #[serde(rename = "CN")]
    Cn,
    #[serde(rename = "CX")]
    Cx,
    #[serde(rename = "CC")]
    Cc,
    #[serde(rename = "CO")]
    Co,
    #[serde(rename = "KM")]
    Km,
    #[serde(rename = "CG")]
    Cg,
    #[serde(rename = "CD")]
    Cd,
    #[serde(rename = "CK")]
    Ck,
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "CI")]
    Ci,
    #[serde(rename = "HR")]
    Hr,
    #[serde(rename = "CU")]
    Cu,
    #[serde(rename = "CY")]
    Cy,
    #[serde(rename = "CZ")]
    Cz,
    #[serde(rename = "DK")]
    Dk,
    #[serde(rename = "DJ")]
    Dj,
    #[serde(rename = "DM")]
    Dm,
    #[serde(rename = "DO")]
    Do,
    #[serde(rename = "EC")]
    Ec,
    #[serde(rename = "EG")]
    Eg,
    #[serde(rename = "SV")]
    Sv,
    #[serde(rename = "GQ")]
    Gq,
    #[serde(rename = "ER")]
    Er,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "ET")]
    Et,
    #[serde(rename = "FK")]
    Fk,
    #[serde(rename = "FO")]
    Fo,
    #[serde(rename = "FJ")]
    Fj,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "GF")]
    Gf,
    #[serde(rename = "PF")]
    Pf,
    #[serde(rename = "TF")]
    Tf,
    #[serde(rename = "GA")]
    Ga,
    #[serde(rename = "GM")]
    Gm,
    #[serde(rename = "GE")]
    Ge,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "GH")]
    Gh,
    #[serde(rename = "GI")]
    Gi,
    #[serde(rename = "GR")]
    Gr,
    #[serde(rename = "GL")]
    Gl,
    #[serde(rename = "GD")]
    Gd,
    #[serde(rename = "GP")]
    Gp,
    #[serde(rename = "GU")]
    Gu,
    #[serde(rename = "GT")]
    Gt,
    #[serde(rename = "GG")]
    Gg,
    #[serde(rename = "GN")]
    Gn,
    #[serde(rename = "GW")]
    Gw,
    #[serde(rename = "GY")]
    Gy,
    #[serde(rename = "HT")]
    Ht,
    #[serde(rename = "HM")]
    Hm,
    #[serde(rename = "VA")]
    Va,
    #[serde(rename = "HN")]
    Hn,
    #[serde(rename = "HK")]
    Hk,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "IS")]
    Is,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IR")]
    Ir,
    #[serde(rename = "IQ")]
    Iq,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "IM")]
    Im,
    #[serde(rename = "IL")]
    Il,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "JM")]
    Jm,
    #[serde(rename = "JP")]
    Jp,
    #[serde(rename = "JE")]
    Je,
    #[serde(rename = "JO")]
    Jo,
    #[serde(rename = "KZ")]
    Kz,
    #[serde(rename = "KE")]
    Ke,
    #[serde(rename = "KI")]
    Ki,
    #[serde(rename = "KR")]
    Kr,
    #[serde(rename = "KW")]
    Kw,
    #[serde(rename = "KG")]
    Kg,
    #[serde(rename = "LA")]
    La,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "LB")]
    Lb,
    #[serde(rename = "LS")]
    Ls,
    #[serde(rename = "LR")]
    Lr,
    #[serde(rename = "LY")]
    Ly,
    #[serde(rename = "LI")]
    Li,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LU")]
    Lu,
    #[serde(rename = "MO")]
    Mo,
    #[serde(rename = "MK")]
    Mk,
    #[serde(rename = "MG")]
    Mg,
    #[serde(rename = "MW")]
    Mw,
    #[serde(rename = "MY")]
    My,
    #[serde(rename = "MV")]
    Mv,
    #[serde(rename = "ML")]
    Ml,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "MH")]
    Mh,
    #[serde(rename = "MQ")]
    Mq,
    #[serde(rename = "MR")]
    Mr,
    #[serde(rename = "MU")]
    Mu,
    #[serde(rename = "YT")]
    Yt,
    #[serde(rename = "MX")]
    Mx,
    #[serde(rename = "FM")]
    Fm,
    #[serde(rename = "MD")]
    Md,
    #[serde(rename = "MC")]
    Mc,
    #[serde(rename = "MN")]
    Mn,
    #[serde(rename = "ME")]
    Me,
    #[serde(rename = "MS")]
    Ms,
    #[serde(rename = "MA")]
    Ma,
    #[serde(rename = "MZ")]
    Mz,
    #[serde(rename = "MM")]
    Mm,
    #[serde(rename = "NA")]
    Na,
    #[serde(rename = "NR")]
    Nr,
    #[serde(rename = "NP")]
    Np,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "AN")]
    An,
    #[serde(rename = "NC")]
    Nc,
    #[serde(rename = "NZ")]
    Nz,
    #[serde(rename = "NI")]
    Ni,
    #[serde(rename = "NE")]
    Ne,
    #[serde(rename = "NG")]
    Ng,
    #[serde(rename = "NU")]
    Nu,
    #[serde(rename = "NF")]
    Nf,
    #[serde(rename = "MP")]
    Mp,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "OM")]
    Om,
    #[serde(rename = "PK")]
    Pk,
    #[serde(rename = "PW")]
    Pw,
    #[serde(rename = "PS")]
    Ps,
    #[serde(rename = "PA")]
    Pa,
    #[serde(rename = "PG")]
    Pg,
    #[serde(rename = "PY")]
    Py,
    #[serde(rename = "PE")]
    Pe,
    #[serde(rename = "PH")]
    Ph,
    #[serde(rename = "PN")]
    Pn,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "PR")]
    Pr,
    #[serde(rename = "QA")]
    Qa,
    #[serde(rename = "RE")]
    Re,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RU")]
    Ru,
    #[serde(rename = "RW")]
    Rw,
    #[serde(rename = "BL")]
    Bl,
    #[serde(rename = "SH")]
    Sh,
    #[serde(rename = "KN")]
    Kn,
    #[serde(rename = "LC")]
    Lc,
    #[serde(rename = "MF")]
    Mf,
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "VC")]
    Vc,
    #[serde(rename = "WS")]
    Ws,
    #[serde(rename = "SM")]
    Sm,
    #[serde(rename = "ST")]
    St,
    #[serde(rename = "SA")]
    Sa,
    #[serde(rename = "SN")]
    Sn,
    #[serde(rename = "RS")]
    Rs,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "SL")]
    Sl,
    #[serde(rename = "SG")]
    Sg,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SI")]
    Si,
    #[serde(rename = "SB")]
    Sb,
    #[serde(rename = "SO")]
    So,
    #[serde(rename = "ZA")]
    Za,
    #[serde(rename = "GS")]
    Gs,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "LK")]
    Lk,
    #[serde(rename = "SD")]
    Sd,
    #[serde(rename = "SR")]
    Sr,
    #[serde(rename = "SJ")]
    Sj,
    #[serde(rename = "SZ")]
    Sz,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "CH")]
    Ch,
    #[serde(rename = "SY")]
    Sy,
    #[serde(rename = "TW")]
    Tw,
    #[serde(rename = "TJ")]
    Tj,
    #[serde(rename = "TZ")]
    Tz,
    #[serde(rename = "TH")]
    Th,
    #[serde(rename = "TL")]
    Tl,
    #[serde(rename = "TG")]
    Tg,
    #[serde(rename = "TK")]
    Tk,
    #[serde(rename = "TO")]
    To,
    #[serde(rename = "TT")]
    Tt,
    #[serde(rename = "TN")]
    Tn,
    #[serde(rename = "TR")]
    Tr,
    #[serde(rename = "TM")]
    Tm,
    #[serde(rename = "TC")]
    Tc,
    #[serde(rename = "TV")]
    Tv,
    #[serde(rename = "UG")]
    Ug,
    #[serde(rename = "UA")]
    Ua,
    #[serde(rename = "AE")]
    Ae,
    #[serde(rename = "GB")]
    Gb,
    #[serde(rename = "US")]
    Us,
    #[serde(rename = "UM")]
    Um,
    #[serde(rename = "UY")]
    Uy,
    #[serde(rename = "UZ")]
    Uz,
    #[serde(rename = "VU")]
    Vu,
    #[serde(rename = "VE")]
    Ve,
    #[serde(rename = "VN")]
    Vn,
    #[serde(rename = "VG")]
    Vg,
    #[serde(rename = "VI")]
    Vi,
    #[serde(rename = "WF")]
    Wf,
    #[serde(rename = "EH")]
    Eh,
    #[serde(rename = "YE")]
    Ye,
    #[serde(rename = "ZM")]
    Zm,
    #[serde(rename = "ZW")]
    Zw,
}

impl Default for CountryCode {
    fn default() -> CountryCode {
        Self::Af
    }
}
/// The attribute type of the value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AttributeType {
    #[serde(rename = "phone-number")]
    PhoneNumber,
}

impl Default for AttributeType {
    fn default() -> AttributeType {
        Self::PhoneNumber
    }
}

