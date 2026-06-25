pub mod p0001;
pub mod p0002;
pub mod p0003;
pub mod p0004;
pub mod p0005;
pub mod p0006;
pub mod p0007;
pub mod p0008;
pub mod p0009;
pub mod p0010;
pub mod p0011;
pub mod p0012;
pub mod p0013;
pub mod p0014;
pub mod p0015;
pub mod p0016;
pub mod p0017;
pub mod p0018;
pub mod p0019;
pub mod p0020;
pub mod p0021;
pub mod p0022;
pub mod p0023;
pub mod p0024;
pub mod p0025;
pub mod p0026;
pub mod p0027;
pub mod p0028;
pub mod p0029;
pub mod p0030;
pub mod p0031;
pub mod p0032;
pub mod p0033;
pub mod p0034;
pub mod p0035;
pub mod p0036;
pub mod p0037;
pub mod p0038;
pub mod p0039;
pub mod p0040;
pub mod p0041;
pub mod p0042;
pub mod p0043;
pub mod p0044;
pub mod p0045;
pub mod p0046;
pub mod p0047;
pub mod p0048;
pub mod p0049;
pub mod p0050;
pub mod p0051;
pub mod p0052;
pub mod p0053;
pub mod p0054;
pub mod p0055;
pub mod p0056;
pub mod p0057;
pub mod p0058;
pub mod p0059;
pub mod p0060;
pub mod p0061;
pub mod p0062;
pub mod p0063;
pub mod p0064;
pub mod p0065;
pub mod p0066;
pub mod p0067;
pub mod p0068;
pub mod p0069;
pub mod p0070;
pub mod p0071;
pub mod p0072;
pub mod p0073;
pub mod p0074;
pub mod p0075;
pub mod p0076;
pub mod p0077;
pub mod p0078;
pub mod p0079;
pub mod p0080;
pub mod p0081;
pub mod p0082;
pub mod p0083;
pub mod p0084;
pub mod p0085;
pub mod p0086;
pub mod p0087;
pub mod p0088;
pub mod p0089;
pub mod p0090;
pub mod p0091;
pub mod p0092;
pub mod p0093;
pub mod p0094;
pub mod p0095;
pub mod p0096;
pub mod p0097;
pub mod p0098;
pub mod p0099;
pub mod p0100;

pub type Solver = fn() -> u64;

pub const SOLVERS: &[Solver] = &[
    p0001::solve,
    p0002::solve,
    p0003::solve,
    p0004::solve,
    p0005::solve,
    p0006::solve,
    p0007::solve,
    p0008::solve,
    p0009::solve,
    p0010::solve,
    p0011::solve,
    p0012::solve,
    p0013::solve,
    p0014::solve,
    p0015::solve,
    p0016::solve,
    p0017::solve,
    p0018::solve,
    p0019::solve,
    p0020::solve,
    p0021::solve,
    p0022::solve,
    p0023::solve,
    p0024::solve,
    p0025::solve,
    p0026::solve,
    p0027::solve,
    p0028::solve,
    p0029::solve,
    p0030::solve,
    p0031::solve,
    p0032::solve,
    p0033::solve,
    p0034::solve,
    p0035::solve,
    p0036::solve,
    p0037::solve,
    p0038::solve,
    p0039::solve,
    p0040::solve,
    p0041::solve,
    p0042::solve,
    p0043::solve,
    p0044::solve,
    p0045::solve,
    p0046::solve,
    p0047::solve,
    p0048::solve,
    p0049::solve,
    p0050::solve,
    p0051::solve,
    p0052::solve,
    p0053::solve,
    p0054::solve,
    p0055::solve,
    p0056::solve,
    p0057::solve,
    p0058::solve,
    p0059::solve,
    p0060::solve,
    p0061::solve,
    p0062::solve,
    p0063::solve,
    p0064::solve,
    p0065::solve,
    p0066::solve,
    p0067::solve,
    p0068::solve,
    p0069::solve,
    p0070::solve,
    p0071::solve,
    p0072::solve,
    p0073::solve,
    p0074::solve,
    p0075::solve,
    p0076::solve,
    p0077::solve,
    p0078::solve,
    p0079::solve,
    p0080::solve,
    p0081::solve,
    p0082::solve,
    p0083::solve,
    p0084::solve,
    p0085::solve,
    p0086::solve,
    p0087::solve,
    p0088::solve,
    p0089::solve,
    p0090::solve,
    p0091::solve,
    p0092::solve,
    p0093::solve,
    p0094::solve,
    p0095::solve,
    p0096::solve,
    p0097::solve,
    p0098::solve,
    p0099::solve,
    p0100::solve,
];
