use combine::*;
use lattice::*;
use to_str::to_str;

enum Lub = combine_fields;  // "subtype", "subregion" etc

impl Lub: combine {
    fn infcx() -> infer_ctxt { self.infcx }
    fn tag() -> ~str { ~"lub" }
    fn a_is_expected() -> bool { self.a_is_expected }

    fn sub() -> Sub { Sub(*self) }
    fn lub() -> Lub { Lub(*self) }
    fn glb() -> Glb { Glb(*self) }

    fn bot_ty(b: ty::t) -> cres<ty::t> { Ok(b) }
    fn ty_bot(b: ty::t) -> cres<ty::t> { self.bot_ty(b) } // commutative

    fn mts(a: ty::mt, b: ty::mt) -> cres<ty::mt> {
        let tcx = self.infcx.tcx;

        debug!("%s.mts(%s, %s)",
               self.tag(),
               mt_to_str(tcx, a),
               mt_to_str(tcx, b));

        let m = if a.mutbl == b.mutbl {
            a.mutbl
        } else {
            m_const
        };

        match m {
          m_imm | m_const => {
            self.tys(a.ty, b.ty).chain(|t| Ok({ty: t, mutbl: m}) )
          }

          m_mutbl => {
            self.infcx.try(|| {
                eq_tys(&self, a.ty, b.ty).then(|| {
                    Ok({ty: a.ty, mutbl: m})
                })
            }).chain_err(|_e| {
                self.tys(a.ty, b.ty).chain(|t| {
                    Ok({ty: t, mutbl: m_const})
                })
            })
          }
        }
    }

    fn contratys(a: ty::t, b: ty::t) -> cres<ty::t> {
        Glb(*self).tys(a, b)
    }

    // XXX: Wrong.
    fn protos(p1: ty::fn_proto, p2: ty::fn_proto) -> cres<ty::fn_proto> {
        match (p1, p2) {
            (ty::proto_bare, _) => Ok(p2),
            (_, ty::proto_bare) => Ok(p1),
            (ty::proto_vstore(v1), ty::proto_vstore(v2)) => {
                self.infcx.try(|| {
                    do self.vstores(terr_fn, v1, v2).chain |vs| {
                        Ok(ty::proto_vstore(vs))
                    }
                }).chain_err(|_err| {
                    // XXX: Totally unsound, but fixed up later.
                    Ok(ty::proto_vstore(ty::vstore_slice(ty::re_static)))
                })
            }
        }
    }

    fn purities(a: purity, b: purity) -> cres<purity> {
        match (a, b) {
          (unsafe_fn, _) | (_, unsafe_fn) => Ok(unsafe_fn),
          (impure_fn, _) | (_, impure_fn) => Ok(impure_fn),
          (extern_fn, _) | (_, extern_fn) => Ok(extern_fn),
          (pure_fn, pure_fn) => Ok(pure_fn)
        }
    }

    fn ret_styles(r1: ret_style, r2: ret_style) -> cres<ret_style> {
        match (r1, r2) {
          (ast::return_val, _) |
          (_, ast::return_val) => Ok(ast::return_val),
          (ast::noreturn, ast::noreturn) => Ok(ast::noreturn)
        }
    }

    fn contraregions(a: ty::region, b: ty::region) -> cres<ty::region> {
        return Glb(*self).regions(a, b);
    }

    fn regions(a: ty::region, b: ty::region) -> cres<ty::region> {
        debug!("%s.regions(%?, %?)",
               self.tag(),
               a.to_str(self.infcx),
               b.to_str(self.infcx));

        do indent {
            self.infcx.region_vars.lub_regions(self.span, a, b)
        }
    }

    // Traits please (FIXME: #2794):

    fn tys(a: ty::t, b: ty::t) -> cres<ty::t> {
        lattice_tys(&self, a, b)
    }

    fn flds(a: ty::field, b: ty::field) -> cres<ty::field> {
        super_flds(&self, a, b)
    }

    fn vstores(vk: ty::terr_vstore_kind,
               a: ty::vstore, b: ty::vstore) -> cres<ty::vstore> {
        super_vstores(&self, vk, a, b)
    }

    fn modes(a: ast::mode, b: ast::mode) -> cres<ast::mode> {
        super_modes(&self, a, b)
    }

    fn args(a: ty::arg, b: ty::arg) -> cres<ty::arg> {
        super_args(&self, a, b)
    }

    fn fns(a: &ty::FnTy, b: &ty::FnTy) -> cres<ty::FnTy> {
        super_fns(&self, a, b)
    }

    fn fn_metas(a: &ty::FnMeta, b: &ty::FnMeta) -> cres<ty::FnMeta> {
        super_fn_metas(&self, a, b)
    }

    fn fn_sigs(a: &ty::FnSig, b: &ty::FnSig) -> cres<ty::FnSig> {
        super_fn_sigs(&self, a, b)
    }

    fn substs(did: ast::def_id,
              as: &ty::substs,
              bs: &ty::substs) -> cres<ty::substs> {
        super_substs(&self, did, as, bs)
    }

    fn tps(as: &[ty::t], bs: &[ty::t]) -> cres<~[ty::t]> {
        super_tps(&self, as, bs)
    }

    fn self_tys(a: Option<ty::t>, b: Option<ty::t>) -> cres<Option<ty::t>> {
        super_self_tys(&self, a, b)
    }
}
