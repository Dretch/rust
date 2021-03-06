use check::fn_ctxt;

// Requires that the two types unify, and prints an error message if they
// don't.
fn suptype(fcx: @fn_ctxt, sp: span,
           expected: ty::t, actual: ty::t) {

    // n.b.: order of actual, expected is reversed
    match infer::mk_subty(fcx.infcx(), false, sp,
                          actual, expected) {
      result::Ok(()) => { /* ok */ }
      result::Err(ref err) => {
        fcx.report_mismatched_types(sp, expected, actual, err);
      }
    }
}

fn eqtype(fcx: @fn_ctxt, sp: span,
          expected: ty::t, actual: ty::t) {

    match infer::mk_eqty(fcx.infcx(), false, sp, actual, expected) {
        Ok(()) => { /* ok */ }
        Err(ref err) => {
            fcx.report_mismatched_types(sp, expected, actual, err);
        }
    }
}

// Checks that the type `actual` can be assigned to `expected`.
fn assign(fcx: @fn_ctxt, sp: span, borrow_lb: ast::node_id,
          expected: ty::t, expr: @ast::expr) {
    let expr_ty = fcx.expr_ty(expr);
    match fcx.mk_assignty(expr, borrow_lb, expr_ty, expected) {
      result::Ok(()) => { /* ok */ }
      result::Err(ref err) => {
        fcx.report_mismatched_types(sp, expected, expr_ty, err);
      }
    }
}


