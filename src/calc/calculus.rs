//! #calculus
//! Numerical methods for integration.
//! 
//! Simple example:
//! 
//! ```rust
//! integrate(f:|x|{2.0*x},a:0.0,b:1.0) // returns 1.0
//! ```
//! 
//! _Tanh-Sinh High-Precision Quadrature_ David H. Bailey, 2006 
//! http://crd-legacy.lbl.gov/~dhbailey/dhbpapers/dhb-tanh-sinh.pdf
//! 

// GENERATED CODE DO NOT EDIT

const WEIGHTS: [f64; 409] = [2.412423038430878e-15,4.289196222067919e-15,7.556721775780599e-15,1.3194342231967934e-14,2.283492670261385e-14,3.917679450601626e-14,6.663992283308776e-14,1.1240208959922889e-13,1.8802071730750654e-13,3.1194977286848145e-13,5.134127524501447e-13,8.383128960502627e-13,1.3581784274539059e-12,2.183592209923368e-12,3.484193767026112e-12,5.5182369468174856e-12,8.675931414979638e-12,1.3542512912336309e-11,2.09893354045114e-11,3.2304464333252424e-11,4.937853877663184e-11,7.496739757381804e-11,1.1306055347494672e-10,1.6939457789411684e-10,2.5216347918530193e-10,3.7299501843052736e-10,5.4828357797095e-10,8.009978447972987e-10,1.163116581425581e-09,1.6788897682161947e-09,2.4091773256476002e-09,3.437185674465006e-09,4.8760060974240764e-09,6.878461095589907e-09,9.649888896108964e-09,1.346464552230203e-08,1.8687282268736427e-08,2.5799568229535887e-08,3.5434777171421937e-08,4.842095019807234e-08,6.583518512718343e-08,8.90713951402423e-08,1.199244278290279e-07,1.6069394579076212e-07,2.1431204556943013e-07,2.844992365915972e-07,3.7595411862360666e-07,4.945828870275441e-07,6.477756603592981e-07,8.447375638485988e-07,1.0968835125901286e-06,1.4183067155493914e-06,1.8263320593710639e-06,2.34216672085281e-06,2.9916615878138832e-06,3.8061983264645003e-06,4.823718203261542e-06,6.089910032094918e-06,7.659575852520328e-06,9.598194128378481e-06,1.1983701363170751e-05,1.4908514031870627e-05,1.8481813599879184e-05,2.283211810903611e-05,2.8110164327940182e-05,3.449212475934322e-05,4.2183183841757605e-05,5.142149744765882e-05,6.248255924074409e-05,7.568399658620161e-05,9.139081749071025e-05,0.00011002112846666715,0.00013205234125609997,0.00015802788400701202,0.0001885644297670032,0.0002243596520500854,0.00026620051375271666,0.00031497209186021226,0.0003716669362167776,0.00043739495615911745,0.0005133938240679038,0.0006010398799114749,0.0007018595156842427,0.0008175410133246952,0.0009499468042834704,0.0011011261134519384,0.0012733279447082377,0.00146901435994298,0.00169087399814264,0.0019418357759843675,0.0022250827064786392,0.0025440657675291738,0.002902517747901318,0.0033044669940348295,0.0037542509774318358,0.004256529599017867,0.004816298143928466,0.005438899797624003,0.006130037632083031,0.006895785969066004,0.00774260102606425,0.008677330749539176,0.009707223739391702,0.010839937168255898,0.012083543599157955,0.013446536605285723,0.014937835096050144,0.01656678625424759,0.01834316698992783,0.020277183817500103,0.022379471063648494,0.024661087314753284,0.027133510013711983,0.029808628117310155,0.03269873272660908,0.03581650560419643,0.039175005493600784,0.0427876521577257,0.046668208054846595,0.050830757572570474,0.055289683742240595,0.060059642358636346,0.0651555334325362,0.07059246990686702,0.07638574357083236,0.08255078811070175,0.08910313924094157,0.09605839186518943,0.10343215422333296,0.11123999898874452,0.11949741128869594,0.12821973363120104,0.1374221077331678,0.14711941325785702,0.15732620348436618,0.16805663794826917,0.17932441211072836,0.19114268413342744,0.2035239988586017,0.21648020911729615,0.23002239451478873,0.24416077786983983,0.25890463951405346,0.27426222968906827,0.29024067931245423,0.306845909417917,0.32408253961152894,0.341953795923017,0.3604614184693437,0.3796055693866516,0.3993847415257171,0.41979566844501565,0.44083323627385823,0.4624903980553677,0.48475809121475555,0.5076251588319083,0.5310782754280542,0.5551018780036335,0.5796781030877877,0.6047867305784036,0.6304051351647438,0.6565082461316277,0.6830685163442638,0.7100559012054691,0.737437848361548,0.7651792989089561,0.7932427008205167,0.8215880352669649,0.8501728564566204,0.8789523455527821,0.907879379154895,0.9369046127456684,0.9659765794123008,0.995041804046133,1.0240449331118116,1.0529288799552667,1.0816349854900704,1.1101031939653405,1.1382722433763053,1.1660798699324344,1.193463025849157,1.220358109579358,1.2467012074518573,1.2724283455378622,1.297475750424978,1.3217801174437729,1.3452788847662513,1.3679105116808963,1.389614759247256,1.410332971446259,1.4300083548723,1.4485862549613222,1.4660144267169655,1.482243297885538,1.497226222541036,1.5109197230741696,1.5232837186347052,1.5342817381543035,1.543881116176959,1.552053169845412,1.5587733555333303,1.5640214037732323,1.5677814313072216,1.5700420292795931,1.5707963267948966,1.5700420292795931,1.5677814313072216,1.5640214037732323,1.5587733555333303,1.552053169845412,1.543881116176959,1.5342817381543035,1.5232837186347052,1.5109197230741696,1.497226222541036,1.482243297885538,1.4660144267169655,1.4485862549613222,1.4300083548723,1.410332971446259,1.389614759247256,1.3679105116808963,1.3452788847662513,1.3217801174437729,1.297475750424978,1.2724283455378622,1.2467012074518573,1.220358109579358,1.193463025849157,1.1660798699324344,1.1382722433763053,1.1101031939653405,1.0816349854900704,1.0529288799552667,1.0240449331118116,0.995041804046133,0.9659765794123008,0.9369046127456684,0.907879379154895,0.8789523455527821,0.8501728564566204,0.8215880352669649,0.7932427008205167,0.7651792989089561,0.737437848361548,0.7100559012054691,0.6830685163442638,0.6565082461316277,0.6304051351647438,0.6047867305784036,0.5796781030877877,0.5551018780036335,0.5310782754280542,0.5076251588319083,0.48475809121475555,0.4624903980553677,0.44083323627385823,0.41979566844501565,0.3993847415257171,0.3796055693866516,0.3604614184693437,0.341953795923017,0.32408253961152894,0.306845909417917,0.29024067931245423,0.27426222968906827,0.25890463951405346,0.24416077786983983,0.23002239451478873,0.21648020911729615,0.2035239988586017,0.19114268413342744,0.17932441211072836,0.16805663794826917,0.15732620348436618,0.14711941325785702,0.1374221077331678,0.12821973363120104,0.11949741128869594,0.11123999898874452,0.10343215422333296,0.09605839186518943,0.08910313924094157,0.08255078811070175,0.07638574357083236,0.07059246990686702,0.0651555334325362,0.060059642358636346,0.055289683742240595,0.050830757572570474,0.046668208054846595,0.0427876521577257,0.039175005493600784,0.03581650560419643,0.03269873272660908,0.029808628117310155,0.027133510013711983,0.024661087314753284,0.022379471063648494,0.020277183817500103,0.01834316698992783,0.01656678625424759,0.014937835096050144,0.013446536605285723,0.012083543599157955,0.010839937168255898,0.009707223739391702,0.008677330749539176,0.00774260102606425,0.006895785969066004,0.006130037632083031,0.005438899797624003,0.004816298143928466,0.004256529599017867,0.0037542509774318358,0.0033044669940348295,0.002902517747901318,0.0025440657675291738,0.0022250827064786392,0.0019418357759843675,0.00169087399814264,0.00146901435994298,0.0012733279447082377,0.0011011261134519384,0.0009499468042834704,0.0008175410133246952,0.0007018595156842427,0.0006010398799114749,0.0005133938240679038,0.00043739495615911745,0.0003716669362167776,0.00031497209186021226,0.00026620051375271666,0.0002243596520500854,0.0001885644297670032,0.00015802788400701202,0.00013205234125609997,0.00011002112846666715,9.139081749071025e-05,7.568399658620161e-05,6.248255924074409e-05,5.142149744765882e-05,4.2183183841757605e-05,3.449212475934322e-05,2.8110164327940182e-05,2.283211810903611e-05,1.8481813599879184e-05,1.4908514031870627e-05,1.1983701363170751e-05,9.598194128378481e-06,7.659575852520328e-06,6.089910032094918e-06,4.823718203261542e-06,3.8061983264645003e-06,2.9916615878138832e-06,2.34216672085281e-06,1.8263320593710639e-06,1.4183067155493914e-06,1.0968835125901286e-06,8.447375638485988e-07,6.477756603592981e-07,4.945828870275441e-07,3.7595411862360666e-07,2.844992365915972e-07,2.1431204556943013e-07,1.6069394579076212e-07,1.199244278290279e-07,8.90713951402423e-08,6.583518512718343e-08,4.842095019807234e-08,3.5434777171421937e-08,2.5799568229535887e-08,1.8687282268736427e-08,1.346464552230203e-08,9.649888896108964e-09,6.878461095589907e-09,4.8760060974240764e-09,3.437185674465006e-09,2.4091773256476002e-09,1.6788897682161947e-09,1.163116581425581e-09,8.009978447972987e-10,5.4828357797095e-10,3.7299501843052736e-10,2.5216347918530193e-10,1.6939457789411684e-10,1.1306055347494672e-10,7.496739757381804e-11,4.937853877663184e-11,3.2304464333252424e-11,2.09893354045114e-11,1.3542512912336309e-11,8.675931414979638e-12,5.5182369468174856e-12,3.484193767026112e-12,2.183592209923368e-12,1.3581784274539059e-12,8.383128960502627e-13,5.134127524501447e-13,3.1194977286848145e-13,1.8802071730750654e-13,1.1240208959922889e-13,6.663992283308776e-14,3.917679450601626e-14,2.283492670261385e-14,1.3194342231967934e-14,7.556721775780599e-15,4.289196222067919e-15,2.412423038430878e-15,];
const ABSCISSA: [f64; 409] = [-0.9999999999999999,-0.9999999999999999,-0.9999999999999998,-0.9999999999999997,-0.9999999999999993,-0.9999999999999989,-0.9999999999999981,-0.9999999999999967,-0.9999999999999944,-0.9999999999999906,-0.9999999999999842,-0.9999999999999739,-0.999999999999957,-0.9999999999999298,-0.9999999999998863,-0.9999999999998171,-0.9999999999997081,-0.9999999999995373,-0.9999999999992715,-0.9999999999988612,-0.9999999999982322,-0.9999999999972741,-0.9999999999958246,-0.9999999999936463,-0.9999999999903939,-0.9999999999855688,-0.9999999999784553,-0.9999999999680332,-0.9999999999528565,-0.9999999999308884,-0.9999999998992777,-0.9999999998540561,-0.9999999997897329,-0.9999999996987544,-0.9999999995707878,-0.9999999993917769,-0.9999999991427051,-0.9999999987979835,-0.999999998323362,-0.9999999976732368,-0.9999999967871991,-0.9999999955856336,-0.9999999939641342,-0.999999991786456,-0.9999999888756649,-0.9999999850030763,-0.9999999798745032,-0.999999973113236,-0.9999999642390809,-0.9999999526426645,-0.9999999375540783,-0.9999999180047947,-0.9999998927816124,-0.9999998603712146,-0.9999998188937128,-0.9999997660233324,-0.9999996988941526,-0.9999996139885502,-0.9999995070057195,-0.9999993727073354,-0.9999992047371147,-0.9999989954106899,-0.9999987354718659,-0.9999984138109648,-0.9999980171405954,-0.9999975296238052,-0.9999969324491904,-0.9999962033471662,-0.9999953160412205,-0.9999942396276167,-0.9999929378766629,-0.9999913684483449,-0.9999894820148185,-0.9999872212820007,-0.9999845199022708,-0.9999813012701207,-0.9999774771924616,-0.9999729464252323,-0.9999675930679435,-0.9999612848078566,-0.9999538710056279,-0.9999451806144587,-0.9999350199250824,-0.9999231701292893,-0.999909384695144,-0.9998933865475925,-0.9998748650487803,-0.9998534727731114,-0.9998288220728749,-0.9998004814311384,-0.9997679715995609,-0.9997307615198084,-0.9996882640283532,-0.9996398313456004,-0.9995847503515176,-0.9995222376512172,-0.9994514344352746,-0.9993714011409377,-0.9992811119217919,-0.999179448934886,-0.9990651964557858,-0.9989370348335121,-0.9987935342988059,-0.9986331486406774,-0.9984542087676977,-0.9982549161719962,-0.9980333363154338,-0.9977873919589065,-0.9975148564572244,-0.9972133470434688,-0.9968803181281919,-0.9965130546402537,-0.9961086654375085,-0.9956640768169531,-0.9951760261553273,-0.9946410557125112,-0.9940555066314022,-0.993415513169264,-0.9927169971968273,-0.9919556630026776,-0.9911269924416988,-0.9902262404675277,-0.9892484310901339,-0.9881883538007427,-0.9870405605073769,-0.9857993630252835,-0.9844588311674308,-0.9830127914811011,-0.9814548266773352,-0.9797782758006157,-0.977976235186665,-0.9760415602565767,-0.9739668681956775,-0.9717445415654873,-0.9693667328969173,-0.9668253703123558,-0.9641121642235473,-0.9612186151511164,-0.9581360227102137,-0.9548554958050227,-0.9513679640727469,-0.9476641906151531,-0.9437347860527572,-0.9395702239332747,-0.9351608575219846,-0.9304969379971534,-0.9255686340686127,-0.9203660530319527,-0.9148792632645746,-0.9090983181630204,-0.9030132815135739,-0.896614254280076,-0.8898914027842602,-0.882834988244669,-0.8754353976304087,-0.8676831757756459,-0.8595690586898966,-0.8510840079878488,-0.8422192463507568,-0.8329662939194109,-0.8233170055064024,-0.8132636085029739,-0.8027987413432413,-0.791915492376142,-0.7806074389832003,-0.7688686867682466,-0.75669390863373,-0.7440783835473472,-0.7310180347925614,-0.7175094674873241,-0.703550005147142,-0.6891377250616676,-0.6742714922484359,-0.65895099174335,-0.6431767589852047,-0.6269502080510427,-0.6102736575006389,-0.5931503535919531,-0.5755844906351517,-0.5575812282607782,-0.5391467053879677,-0.5202880506912302,-0.5010133893793091,-0.48133184611690505,-0.4612535439395857,-0.44078959903390086,-0.4199521112784471,-0.3987541504672377,-0.3772097381640342,-0.3553338251650745,-0.3331422645776381,-0.3106517805528459,-0.28787993274271595,-0.26484507658344797,-0.24156631953888363,-0.218063473469712,-0.1943570033249354,-0.17046797238201053,-0.14641798429058792,-0.12222912220155766,-0.09792388528783233,-0.07352512298567129,-0.049055967305077885,-0.024539763574649157,0.0,0.024539763574649157,0.049055967305077885,0.07352512298567129,0.09792388528783233,0.12222912220155766,0.14641798429058792,0.17046797238201053,0.1943570033249354,0.218063473469712,0.24156631953888363,0.26484507658344797,0.28787993274271595,0.3106517805528459,0.3331422645776381,0.3553338251650745,0.3772097381640342,0.3987541504672377,0.4199521112784471,0.44078959903390086,0.4612535439395857,0.48133184611690505,0.5010133893793091,0.5202880506912302,0.5391467053879677,0.5575812282607782,0.5755844906351517,0.5931503535919531,0.6102736575006389,0.6269502080510427,0.6431767589852047,0.65895099174335,0.6742714922484359,0.6891377250616676,0.703550005147142,0.7175094674873241,0.7310180347925614,0.7440783835473472,0.75669390863373,0.7688686867682466,0.7806074389832003,0.791915492376142,0.8027987413432413,0.8132636085029739,0.8233170055064024,0.8329662939194109,0.8422192463507568,0.8510840079878488,0.8595690586898966,0.8676831757756459,0.8754353976304087,0.882834988244669,0.8898914027842602,0.896614254280076,0.9030132815135739,0.9090983181630204,0.9148792632645746,0.9203660530319527,0.9255686340686127,0.9304969379971534,0.9351608575219846,0.9395702239332747,0.9437347860527572,0.9476641906151531,0.9513679640727469,0.9548554958050227,0.9581360227102137,0.9612186151511164,0.9641121642235473,0.9668253703123558,0.9693667328969173,0.9717445415654873,0.9739668681956775,0.9760415602565767,0.977976235186665,0.9797782758006157,0.9814548266773352,0.9830127914811011,0.9844588311674308,0.9857993630252835,0.9870405605073769,0.9881883538007427,0.9892484310901339,0.9902262404675277,0.9911269924416988,0.9919556630026776,0.9927169971968273,0.993415513169264,0.9940555066314022,0.9946410557125112,0.9951760261553273,0.9956640768169531,0.9961086654375085,0.9965130546402537,0.9968803181281919,0.9972133470434688,0.9975148564572244,0.9977873919589065,0.9980333363154338,0.9982549161719962,0.9984542087676977,0.9986331486406774,0.9987935342988059,0.9989370348335121,0.9990651964557858,0.999179448934886,0.9992811119217919,0.9993714011409377,0.9994514344352746,0.9995222376512172,0.9995847503515176,0.9996398313456004,0.9996882640283532,0.9997307615198084,0.9997679715995609,0.9998004814311384,0.9998288220728749,0.9998534727731114,0.9998748650487803,0.9998933865475925,0.999909384695144,0.9999231701292893,0.9999350199250824,0.9999451806144587,0.9999538710056279,0.9999612848078566,0.9999675930679435,0.9999729464252323,0.9999774771924616,0.9999813012701207,0.9999845199022708,0.9999872212820007,0.9999894820148185,0.9999913684483449,0.9999929378766629,0.9999942396276167,0.9999953160412205,0.9999962033471662,0.9999969324491904,0.9999975296238052,0.9999980171405954,0.9999984138109648,0.9999987354718659,0.9999989954106899,0.9999992047371147,0.9999993727073354,0.9999995070057195,0.9999996139885502,0.9999996988941526,0.9999997660233324,0.9999998188937128,0.9999998603712146,0.9999998927816124,0.9999999180047947,0.9999999375540783,0.9999999526426645,0.9999999642390809,0.999999973113236,0.9999999798745032,0.9999999850030763,0.9999999888756649,0.999999991786456,0.9999999939641342,0.9999999955856336,0.9999999967871991,0.9999999976732368,0.999999998323362,0.9999999987979835,0.9999999991427051,0.9999999993917769,0.9999999995707878,0.9999999996987544,0.9999999997897329,0.9999999998540561,0.9999999998992777,0.9999999999308884,0.9999999999528565,0.9999999999680332,0.9999999999784553,0.9999999999855688,0.9999999999903939,0.9999999999936463,0.9999999999958246,0.9999999999972741,0.9999999999982322,0.9999999999988612,0.9999999999992715,0.9999999999995373,0.9999999999997081,0.9999999999998171,0.9999999999998863,0.9999999999999298,0.999999999999957,0.9999999999999739,0.9999999999999842,0.9999999999999906,0.9999999999999944,0.9999999999999967,0.9999999999999981,0.9999999999999989,0.9999999999999993,0.9999999999999997,0.9999999999999998,0.9999999999999999,0.9999999999999999,];
const H: f64 = 0.015625;

// END GENERATED CODE

/// integrate takes an arbitrary univariate function, a start and end point and
/// then performs an arbitrary precision numerical calculation of an integral.
/// 
/// A future implementation might attempt to make a reasonable inference about
/// precision and save computational time by increasing the size of the stride
/// through the array of abscissa and weights.
pub fn integrate(f: fn(x:f64)->f64, a: f64, b: f64) -> f64 {
    let mut sum: f64 = 0.0;
    for i in 0..ABSCISSA.len() {
        sum += f((b+a)/2.0 + (b-a)/2.0*ABSCISSA[i])*WEIGHTS[i]
    }
    return (b+a)/2.0 * H * sum;
}

#[cfg(test)]
mod tests {
    use super::integrate;
    const PRECISION: f64 = 1.0e-6;

    #[test]    
    fn int_basic() {
        assert!(integrate(|x|{x},0.0,1.0) - 0.5 < PRECISION)
    }

    #[test]
    fn table_test() {
        struct Row {
            args: Args,
            expected: f64,
        }

        struct Args {
            f: fn(x:f64)->f64,
            a: f64,
            b: f64,
        }
        let table: Vec<Row> = vec![
            Row{
                args:Args{
                    f:|x|{x*x},
                    a:-1.0,
                    b:1.0
                },
                expected:0.0
            },
            Row{args:Args{f:|x|{2.0*x + 5.0},a:0.0,b:10.0},expected:150.0},
        ];
        for r in table.iter() {
            assert!(integrate(r.args.f,r.args.a,r.args.b) - r.expected < PRECISION)
        }
    }
}