use super::DConv;
use gf::{gf, GF};
use intalgo::prime::primitive_root;
/// This is NTT which is convolution of Z/pZ.
trait NTTPrecalc {
    fn precalc() -> &'static ([u32; 30], [u32; 30]);
}
impl<const MOD: u32> NTTPrecalc for GF<MOD> {
    fn precalc() -> &'static ([u32; 30], [u32; 30]) {
        use std::sync::OnceLock;
        static NTT_PRECALC_LOCK: OnceLock<([u32; 30], [u32; 30])> = OnceLock::new();
        NTT_PRECALC_LOCK.get_or_init(|| {
            let k = (MOD - 1).trailing_zeros() as usize;
            assert!(k < 30);
            let g = GF::<MOD>::new(primitive_root(MOD as u64) as u32);
            let mut omega = g.pow((MOD - 1) >> k);
            let mut iomega = omega.inv();
            let mut ws = [gf!(0); 30];
            let mut iws = [gf!(0); 30];
            ws[..k - 1]
                .iter_mut()
                .zip(iws[..k - 1].iter_mut())
                .rev()
                .for_each(|(w, iw)| {
                    *w = omega;
                    *iw = iomega;
                    omega *= omega;
                    iomega *= iomega;
                });
            let mut ss = [0; 30];
            let mut iss = [0; 30];
            let mut zeta = gf!(1);
            let mut izeta = gf!(1);
            ss[..k - 1]
                .iter_mut()
                .zip(iss[..k - 1].iter_mut())
                .zip(ws.iter())
                .zip(iws.iter())
                .for_each(|(((s, is), w), iw)| {
                    *s = (w * zeta).rep();
                    *is = (iw * izeta).rep();
                    zeta *= iw;
                    izeta *= w;
                });
            (ss, iss)
        })
    }
}
impl<const MOD: u32> DConv for [GF<MOD>] {
    type S = GF<MOD>;
    #[inline]
    fn ft(&mut self) {
        use std::iter::successors;
        let n = self.len();
        let (ss, _) = <GF<MOD>>::precalc();
        successors(Some(n >> 1), |i| Some(i >> 1))
            .take_while(|i| *i > 0)
            .for_each(|i| {
                let mut c = GF::<MOD>::new(1);
                self.chunks_exact_mut(2 * i).enumerate().for_each(|(t, a)| {
                    let (x, y) = a.split_at_mut(i);
                    x.iter_mut().zip(y.iter_mut()).for_each(|(x, y)| {
                        (*x, *y) = (*x + *y * c, *x - *y * c);
                    });
                    c *= gf!(ss[(!t).trailing_zeros() as usize]);
                });
            })
    }
    #[inline]
    fn ift(&mut self) {
        use std::iter::successors;
        let n = self.len();
        let (_, iss) = <GF<MOD>>::precalc();
        successors(Some(1), |i| Some(i << 1))
            .take_while(|i| *i < n)
            .for_each(|i| {
                let mut c = GF::<MOD>::new(1);
                self.chunks_exact_mut(2 * i).enumerate().for_each(|(t, a)| {
                    let (x, y) = a.split_at_mut(i);
                    x.iter_mut().zip(y.iter_mut()).for_each(|(x, y)| {
                        (*x, *y) = (*x + *y, (*x - *y) * c);
                    });
                    c *= gf!(iss[(!t).trailing_zeros() as usize]);
                });
            });
        let c = GF::<MOD>::new(2).inv().pow(n.trailing_zeros());
        for a in self.iter_mut() {
            *a *= c;
        }
    }
    fn conv(&self, rhs: &[GF<MOD>]) -> Vec<GF<MOD>> {
        let n = (self.len() + rhs.len() - 1).next_power_of_two();
        let mut f = vec![GF::<MOD>::new(0); n];
        let mut g = vec![GF::<MOD>::new(0); n];
        f[..self.len()].copy_from_slice(self);
        g[..rhs.len()].copy_from_slice(rhs);
        f.ft();
        g.ft();
        let mut h = f
            .iter()
            .zip(g.iter())
            .map(|(f, g)| f * g)
            .collect::<Vec<_>>();
        h.ift();
        h.truncate(self.len() + rhs.len() - 1);
        h
    }
}
