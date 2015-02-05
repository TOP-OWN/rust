// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*!
 * This module contains a simple utility routine
 * used by both `typeck` and `const_eval`.
 * Almost certainly this could (and should) be refactored out of existence.
 */

use middle::def;
use middle::ty::{self, Ty};
use syntax::ast;
use syntax::codemap::Span;
use util::ppaux::Repr;

pub const NO_REGIONS: uint = 1;
pub const NO_TPS: uint = 2;

pub fn check_path_args(tcx: &ty::ctxt,
                       span: Span,
                       segments: &[ast::PathSegment],
                       flags: uint) {
    if (flags & NO_TPS) != 0 {
        if segments.iter().any(|s| s.parameters.has_types()) {
            span_err!(tcx.sess, span, E0109,
                "type parameters are not allowed on this type");
        }
    }

    if (flags & NO_REGIONS) != 0 {
        if segments.iter().any(|s| s.parameters.has_lifetimes()) {
            span_err!(tcx.sess, span, E0110,
                "lifetime parameters are not allowed on this type");
        }
    }
}

pub fn prim_ty_to_ty<'tcx>(tcx: &ty::ctxt<'tcx>,
                           span: Span,
                           segments: &[ast::PathSegment],
                           nty: ast::PrimTy)
                           -> Ty<'tcx> {
    check_path_args(tcx, span, segments, NO_TPS | NO_REGIONS);
    match nty {
        ast::TyBool => tcx.types.bool,
        ast::TyChar => tcx.types.char,
        ast::TyInt(it) => ty::mk_mach_int(tcx, it),
        ast::TyUint(uit) => ty::mk_mach_uint(tcx, uit),
        ast::TyFloat(ft) => ty::mk_mach_float(tcx, ft),
        ast::TyStr => ty::mk_str(tcx)
    }
}

pub fn ast_ty_to_prim_ty<'tcx>(tcx: &ty::ctxt<'tcx>, ast_ty: &ast::Ty)
                               -> Option<Ty<'tcx>> {
    if let ast::TyPath(ref path) = ast_ty.node {
        let def = match tcx.def_map.borrow().get(&ast_ty.id) {
            None => {
                tcx.sess.span_bug(ast_ty.span,
                                  &format!("unbound path {}", path.repr(tcx)))
            }
            Some(&d) => d
        };
        if let def::DefPrimTy(nty) = def {
            Some(prim_ty_to_ty(tcx, path.span, &path.segments[], nty))
        } else {
            None
        }
    } else {
        None
    }
}

