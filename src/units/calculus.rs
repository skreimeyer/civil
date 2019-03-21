//! Calculus utility to be used in other parts of the library.
//! # THERE IS ZERO ERROR HANDLING HERE #
//! # Everything is going to be heavily refactored before 1.0 #

/// These constants are used for Gauss-Legendre Quadrature calculations. We
/// have the option of solving for equations up to the order of 127, which is
/// probably never necessary. So far, these solutions seem to produce very
/// adequate results. At some point these methods should be refactored for
/// either:
/// - symbolic integration
/// - more robust methodology
/// I suspect this will be 'good enough' precision for almost all practical
/// situations.
pub const ABSCISSA: [f64;224] = [-0.5773502691896257, 0.5773502691896257, -0.7745966692414834, 0.0, 0.7745966692414834, -0.8611363115940526, -0.33998104358485626, 0.33998104358485626, 0.8611363115940526, -0.906179845938664, -0.5384693101056831, 0.0, 0.5384693101056831, 0.906179845938664, -0.932469514203152, -0.6612093864662645, -0.2386191860831969, 0.2386191860831969, 0.6612093864662645, 0.932469514203152, -0.9491079123427585, -0.7415311855993945, -0.4058451513773972, 0.0, 0.4058451513773972, 0.7415311855993945, 0.9491079123427585, -0.9602898564975363, -0.7966664774136267, -0.525532409916329, -0.1834346424956498, 0.1834346424956498, 0.525532409916329, 0.7966664774136267, 0.9602898564975363, -0.9681602395076261, -0.8360311073266358, -0.6133714327005904, -0.3242534234038089, 0.0, 0.3242534234038089, 0.6133714327005904, 0.8360311073266358, 0.9681602395076261, -0.14887433898163122, 0.0, -0.1252334085114689, 0.0, -0.10805494870734367, 0.0, -0.09501250983763744, 0.0, -0.0847750130417353, 0.0, -0.07652652113349734, 0.07652652113349734, -0.1455618541608951, 0.0, -0.06973927331972223, 0.06973927331972223, -0.1332568242984661, 0.0, -0.06405689286260563, 0.06405689286260563, -0.1228646926107104, 0.0, -0.05923009342931321, 0.05923009342931321, -0.11397258560952997, 0.0, -0.05507928988403427, 0.05507928988403427, -0.10627823013267923, 0.0, -0.15386991360858354, -0.0514718425553177, 0.0514718425553177, -0.09955531215234152, 0.0, 0.09955531215234152, -0.1444719615827965, -0.04830766568773832, 0.04830766568773832, -0.09363106585473338, 0.0, 0.09363106585473338, -0.13615235725918298, -0.04550982195310254, 0.04550982195310254, -0.08837134327565926, 0.0, 0.08837134327565926, -0.1287361038093848, -0.04301819847370861, 0.04301819847370861, -0.0836704089547699, 0.0, 0.0836704089547699, -0.12208402533786741, -0.04078514790457824, 0.04078514790457824, -0.07944380460875548, 0.0, 0.07944380460875548, -0.11608407067525521, -0.03877241750605082, 0.03877241750605082, 0.11608407067525521, -0.15081335486399217, -0.075623258989163, 0.0, 0.075623258989163, -0.11064502720851987, -0.03694894316535178, 0.03694894316535178, 0.11064502720851987, -0.14392980951071332, -0.07215299087458624, 0.0, 0.07215299087458624, -0.10569190170865325, -0.035289236964135356, 0.035289236964135356, 0.10569190170865325, -0.13764520598325303, -0.06898698016314417, 0.0, 0.06898698016314417, -0.10116247530558424, -0.03377219001605204, 0.03377219001605204, 0.10116247530558424, -0.1318848665545149, -0.06608692391635568, 0.0, 0.06608692391635568, -0.0970046992094627, -0.03238017096286936, 0.03238017096286936, 0.0970046992094627, -0.12658599726967204, -0.06342068498268678, 0.0, 0.06342068498268678, -0.1548905899981459, -0.09317470156008614, -0.031098338327188876, 0.031098338327188876, 0.09317470156008614, -0.12169542101888876, -0.06096110015057873, 0.0, 0.06096110015057873, 0.12169542101888876, -0.14903550860694917, -0.08963524464890056, -0.029914109797338766, 0.029914109797338766, 0.08963524464890056, -0.11716780907195515, -0.058685054300259464, 0.0, 0.058685054300259464, 0.11716780907195515, -0.14360542731625614, -0.08635451826324822, -0.02881674819934178, 0.02881674819934178, 0.08635451826324822, -0.11296428805932926, -0.05657275381833678, 0.0, 0.05657275381833678, 0.11296428805932926, -0.13855584681037625, -0.08330518682243537, -0.027797035287275437, 0.027797035287275437, 0.08330518682243537, -0.1090513328087878, -0.05460715100164682, 0.0, 0.05460715100164682, 0.1090513328087878, -0.13384825059546684, -0.08046363021414273, -0.026847012365942356, 0.026847012365942356, 0.08046363021414273, -0.10539987901634415, -0.05277348408831, 0.0, 0.05277348408831, 0.10539987901634415, -0.129449135396945, -0.07780933394953657, -0.0259597723012478, 0.0259597723012478, 0.07780933394953657, 0.129449135396945, -0.1526442402308153, -0.10198460656227407, -0.05105890670797435, 0.0, 0.05105890670797435, 0.10198460656227407, -0.1253292236158968, -0.07532439549623433, -0.025129291421820615, 0.025129291421820615, 0.07532439549623433, 0.1253292236158968, -0.14787278635787196, -0.09878335644694528, -0.049452187116159625, 0.0, 0.049452187116159625, 0.09878335644694528, -0.12146281929612056, -0.07299312178779904, -0.024350292663424433, 0.024350292663424433, 0.07299312178779904, 0.12146281929612056];

pub const WEIGHTS: [f64;224] = [1.0, 1.0, 0.5555555555555556, 0.5555555555555556, 0.8888888888888888, 0.34785484513745385, 0.34785484513745385, 0.6521451548625461, 0.6521451548625461, 0.23692688505618908, 0.23692688505618908, 0.47862867049936647, 0.47862867049936647, 0.5688888888888889, 0.17132449237917036, 0.17132449237917036, 0.3607615730481386, 0.3607615730481386, 0.46791393457269104, 0.46791393457269104, 0.1294849661688697, 0.1294849661688697, 0.27970539148927664, 0.27970539148927664, 0.3818300505051189, 0.3818300505051189, 0.4179591836734694, 0.10122853629037626, 0.10122853629037626, 0.22238103445337448, 0.22238103445337448, 0.31370664587788727, 0.31370664587788727, 0.362683783378362, 0.362683783378362, 0.08127438836157441, 0.08127438836157441, 0.1806481606948574, 0.1806481606948574, 0.26061069640293544, 0.26061069640293544, 0.31234707704000286, 0.31234707704000286, 0.3302393550012598, 0.29552422471475287, 0.2729250867779006, 0.24914704581340277, 0.2325515532308739, 0.2152638534631578, 0.2025782419255613, 0.1894506104550685, 0.17944647035620653, 0.1691423829631436, 0.1610544498487837, 0.15275338713072584, 0.15275338713072584, 0.14452440398997005, 0.14608113364969041, 0.13925187285563198, 0.13925187285563198, 0.1324620394046966, 0.13365457218610619, 0.12793819534675216, 0.12793819534675216, 0.12224244299031004, 0.12317605372671545, 0.11832141527926228, 0.11832141527926228, 0.11347634610896515, 0.114220867378957, 0.1100470130164752, 0.1100470130164752, 0.10587615509732094, 0.10647938171831424, 0.1017623897484055, 0.10285265289355884, 0.10285265289355884, 0.09922501122667231, 0.09922501122667231, 0.09972054479342646, 0.09563872007927486, 0.0965400885147278, 0.0965400885147278, 0.09335642606559612, 0.09335642606559612, 0.09376844616021, 0.09020304437064074, 0.09095674033025987, 0.09095674033025987, 0.08814053043027546, 0.08814053043027546, 0.08848679490710429, 0.08534668573933862, 0.08598327567039475, 0.08598327567039475, 0.08347457362586279, 0.08347457362586279, 0.0837683609931389, 0.0809824937705971, 0.08152502928038578, 0.08152502928038578, 0.07927622256836847, 0.07927622256836847, 0.07952762213944285, 0.07703981816424797, 0.07703981816424797, 0.0775059479784248, 0.0775059479784248, 0.07482962317622155, 0.07547874709271582, 0.07547874709271582, 0.07569553564729838, 0.07346081345346753, 0.07346081345346753, 0.07386423423217288, 0.07386423423217288, 0.07146373425251414, 0.07202750197142198, 0.07202750197142198, 0.07221575169379899, 0.07019768547355822, 0.07019768547355822, 0.07054915778935407, 0.07054915778935407, 0.06838457737866968, 0.06887731697766132, 0.06887731697766132, 0.06904182482923202, 0.06721061360067818, 0.06721061360067818, 0.06751868584903646, 0.06751868584903646, 0.06555737776654974, 0.06599053358881048, 0.06599053358881048, 0.06613512962365548, 0.06446616443595009, 0.06446616443595009, 0.06473769681268392, 0.06473769681268392, 0.0629527074651957, 0.06333550929649175, 0.06333550929649175, 0.0634632814047906, 0.061455899590316665, 0.06193606742068324, 0.06193606742068324, 0.06217661665534726, 0.06217661665534726, 0.06054550693473779, 0.06054550693473779, 0.060885464844856345, 0.060885464844856345, 0.06099892484120588, 0.05916881546604297, 0.05959626017124816, 0.05959626017124816, 0.05981036574529186, 0.05981036574529186, 0.05831431136225601, 0.05831431136225601, 0.058617586232720266, 0.058617586232720266, 0.058718794151164364, 0.0570439735587946, 0.05742613705411211, 0.05742613705411211, 0.057617536707147025, 0.057617536707147025, 0.056240634071084365, 0.056240634071084365, 0.056512318249772, 0.056512318249772, 0.05660297644456042, 0.055064895901762424, 0.05540795250324512, 0.05540795250324512, 0.055579746306514397, 0.055579746306514397, 0.05430847145249864, 0.05430847145249864, 0.05455280360476188, 0.05455280360476188, 0.05463432875658403, 0.05321723644657901, 0.053526343304058255, 0.053526343304058255, 0.05368111986333485, 0.05368111986333485, 0.05250390264782874, 0.05250390264782874, 0.05272443385912793, 0.05272443385912793, 0.05279801262199042, 0.051488451500980935, 0.051488451500980935, 0.05176794317491019, 0.05176794317491019, 0.051907877631220636, 0.051907877631220636, 0.05048247038679741, 0.05081476366881834, 0.05081476366881834, 0.051014487038697265, 0.051014487038697265, 0.05108111944078622, 0.049867528594952394, 0.049867528594952394, 0.05012106956904329, 0.05012106956904329, 0.05024800037525628, 0.05024800037525628, 0.04892845282051199, 0.04923038042374756, 0.04923038042374756, 0.04941183303991818, 0.04941183303991818, 0.04947236662393102, 0.048344762234802954, 0.048344762234802954, 0.04857546744150343, 0.04857546744150343, 0.048690957009139724, 0.048690957009139724];

/// define where in our constants is the index of the first abscissa or weight
/// we want based on our order number
fn find_index(&m: &i32) -> usize {
    ((&m + 1) * &m / 2 - 1) as usize
}

/// Gauss-Legendre quadrature
/// f - polynomial function
/// n - order of function
/// a - start point of the integral
/// b - end point of the integral
/// There is currently no error handling, so edge cases will panic.
/// Expect a patch in the near future returning Result<f64,SomeError>
/// types instead of just f64s.
pub fn integrate(f:fn(f64) -> f64,n:i32,a:f64,b:f64) -> f64 {
    match n {
        0 => f(0.0_f64) * (b - a),
        1 => {
            (b-a)*f((b+a)/2.0)
        },
        2...64 => {
            let mut result = 0.0;
            let legendre_number = (&n + 1) / 2 + 1;
            let start_index = find_index(&legendre_number);
            for k in 0..legendre_number {
                let
                 const_index = (start_index as i32 + k) as usize;
                let transformed_function = f((b-a)/2.0 * ABSCISSA[const_index] + (b+a/2.0));
                result += (b-a)/2.0 * WEIGHTS[const_index] * transformed_function;
            }
            return result;
        },
        _ => 0.0,
    }
}
