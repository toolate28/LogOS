// Lean compiler output
// Module: K22.Syntax
// Imports: Init Mathlib.Algebra.Ring.Basic Mathlib.Algebra.BigOperators.Group.List Mathlib.Data.List.Basic Mathlib.Data.Finset.Basic
#include <lean/lean.h>
#if defined(__clang__)
#pragma clang diagnostic ignored "-Wunused-parameter"
#pragma clang diagnostic ignored "-Wunused-label"
#elif defined(__GNUC__) && !defined(__CLANG__)
#pragma GCC diagnostic ignored "-Wunused-parameter"
#pragma GCC diagnostic ignored "-Wunused-label"
#pragma GCC diagnostic ignored "-Wunused-but-set-variable"
#endif
#ifdef __cplusplus
extern "C" {
#endif
lean_object* l_CommRing_toNonUnitalCommRing___rarg(lean_object*);
LEAN_EXPORT lean_object* l_List_sum___at_K22_exponent__sum___spec__2(lean_object*);
LEAN_EXPORT lean_object* l_List_foldl___at_K22_exponent__sum___spec__3(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_burau___u03c3_u2081___rarg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_instDecidableEqMat2___rarg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l___private_K22_Syntax_0__K22_decEqMat2____x40_K22_Syntax___hyg_44_(lean_object*);
LEAN_EXPORT lean_object* l_List_foldl___at_K22_exponent__sum___spec__3___boxed(lean_object*, lean_object*);
lean_object* l_NonUnitalNonAssocSemiring_toDistrib___rarg(lean_object*);
LEAN_EXPORT lean_object* l_K22_Mat2_mul(lean_object*);
LEAN_EXPORT lean_object* l_K22_burau___u03c3_u2082(lean_object*);
LEAN_EXPORT lean_object* l_K22_instDecidableEqMat2(lean_object*);
LEAN_EXPORT lean_object* l_K22_Mat2_det___rarg(lean_object*, lean_object*);
lean_object* l_NonUnitalNonAssocCommRing_toNonUnitalNonAssocCommSemiring___rarg(lean_object*);
LEAN_EXPORT lean_object* l_List_mapTR_loop___at_K22_exponent__sum___spec__1(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_det__burau___rarg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_Mat2_mul___rarg___boxed(lean_object*, lean_object*, lean_object*);
lean_object* lean_nat_to_int(lean_object*);
LEAN_EXPORT lean_object* l_K22_det__burau(lean_object*);
LEAN_EXPORT lean_object* l_K22_Mat2_instMul(lean_object*);
lean_object* l_CommSemiring_toCommMonoidWithZero___rarg(lean_object*);
LEAN_EXPORT lean_object* l_K22_burau___u03c3_u2081(lean_object*);
LEAN_EXPORT lean_object* l___private_K22_Syntax_0__K22_decEqMat2____x40_K22_Syntax___hyg_44____rarg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_Mat2_instMul___rarg(lean_object*);
LEAN_EXPORT lean_object* l_K22_Mat2_det(lean_object*);
LEAN_EXPORT lean_object* l_K22_burau___u03c3_u2082___rarg(lean_object*, lean_object*);
lean_object* l_List_reverse___rarg(lean_object*);
LEAN_EXPORT lean_object* l_K22_exponent__sum(lean_object*);
lean_object* lean_int_add(lean_object*, lean_object*);
static lean_object* l_List_sum___at_K22_exponent__sum___spec__2___closed__1;
LEAN_EXPORT lean_object* l_K22_Mat2_mul___rarg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l___private_K22_Syntax_0__K22_decEqMat2____x40_K22_Syntax___hyg_44____rarg(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; lean_object* x_10; lean_object* x_11; lean_object* x_12; uint8_t x_13; 
x_4 = lean_ctor_get(x_2, 0);
lean_inc(x_4);
x_5 = lean_ctor_get(x_2, 1);
lean_inc(x_5);
x_6 = lean_ctor_get(x_2, 2);
lean_inc(x_6);
x_7 = lean_ctor_get(x_2, 3);
lean_inc(x_7);
lean_dec(x_2);
x_8 = lean_ctor_get(x_3, 0);
lean_inc(x_8);
x_9 = lean_ctor_get(x_3, 1);
lean_inc(x_9);
x_10 = lean_ctor_get(x_3, 2);
lean_inc(x_10);
x_11 = lean_ctor_get(x_3, 3);
lean_inc(x_11);
lean_dec(x_3);
lean_inc(x_1);
x_12 = lean_apply_2(x_1, x_4, x_8);
x_13 = lean_unbox(x_12);
lean_dec(x_12);
if (x_13 == 0)
{
uint8_t x_14; lean_object* x_15; 
lean_dec(x_11);
lean_dec(x_10);
lean_dec(x_9);
lean_dec(x_7);
lean_dec(x_6);
lean_dec(x_5);
lean_dec(x_1);
x_14 = 0;
x_15 = lean_box(x_14);
return x_15;
}
else
{
lean_object* x_16; uint8_t x_17; 
lean_inc(x_1);
x_16 = lean_apply_2(x_1, x_5, x_9);
x_17 = lean_unbox(x_16);
lean_dec(x_16);
if (x_17 == 0)
{
uint8_t x_18; lean_object* x_19; 
lean_dec(x_11);
lean_dec(x_10);
lean_dec(x_7);
lean_dec(x_6);
lean_dec(x_1);
x_18 = 0;
x_19 = lean_box(x_18);
return x_19;
}
else
{
lean_object* x_20; uint8_t x_21; 
lean_inc(x_1);
x_20 = lean_apply_2(x_1, x_6, x_10);
x_21 = lean_unbox(x_20);
lean_dec(x_20);
if (x_21 == 0)
{
uint8_t x_22; lean_object* x_23; 
lean_dec(x_11);
lean_dec(x_7);
lean_dec(x_1);
x_22 = 0;
x_23 = lean_box(x_22);
return x_23;
}
else
{
lean_object* x_24; 
x_24 = lean_apply_2(x_1, x_7, x_11);
return x_24;
}
}
}
}
}
LEAN_EXPORT lean_object* l___private_K22_Syntax_0__K22_decEqMat2____x40_K22_Syntax___hyg_44_(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l___private_K22_Syntax_0__K22_decEqMat2____x40_K22_Syntax___hyg_44____rarg), 3, 0);
return x_2;
}
}
LEAN_EXPORT lean_object* l_K22_instDecidableEqMat2___rarg(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = l___private_K22_Syntax_0__K22_decEqMat2____x40_K22_Syntax___hyg_44____rarg(x_1, x_2, x_3);
return x_4;
}
}
LEAN_EXPORT lean_object* l_K22_instDecidableEqMat2(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l_K22_instDecidableEqMat2___rarg), 3, 0);
return x_2;
}
}
LEAN_EXPORT lean_object* l_K22_Mat2_mul___rarg(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; lean_object* x_10; lean_object* x_11; lean_object* x_12; lean_object* x_13; lean_object* x_14; lean_object* x_15; lean_object* x_16; lean_object* x_17; lean_object* x_18; lean_object* x_19; lean_object* x_20; lean_object* x_21; lean_object* x_22; lean_object* x_23; lean_object* x_24; lean_object* x_25; lean_object* x_26; lean_object* x_27; lean_object* x_28; lean_object* x_29; 
x_4 = l_CommRing_toNonUnitalCommRing___rarg(x_1);
x_5 = l_NonUnitalNonAssocCommRing_toNonUnitalNonAssocCommSemiring___rarg(x_4);
x_6 = l_NonUnitalNonAssocSemiring_toDistrib___rarg(x_5);
lean_dec(x_5);
x_7 = lean_ctor_get(x_6, 1);
lean_inc(x_7);
lean_dec(x_6);
x_8 = lean_ctor_get(x_4, 1);
lean_inc(x_8);
lean_dec(x_4);
x_9 = lean_ctor_get(x_2, 0);
lean_inc(x_9);
x_10 = lean_ctor_get(x_3, 0);
lean_inc(x_10);
lean_inc(x_8);
lean_inc(x_10);
lean_inc(x_9);
x_11 = lean_apply_2(x_8, x_9, x_10);
x_12 = lean_ctor_get(x_2, 1);
lean_inc(x_12);
x_13 = lean_ctor_get(x_3, 2);
lean_inc(x_13);
lean_inc(x_8);
lean_inc(x_13);
lean_inc(x_12);
x_14 = lean_apply_2(x_8, x_12, x_13);
lean_inc(x_7);
x_15 = lean_apply_2(x_7, x_11, x_14);
x_16 = lean_ctor_get(x_3, 1);
lean_inc(x_16);
lean_inc(x_8);
lean_inc(x_16);
x_17 = lean_apply_2(x_8, x_9, x_16);
x_18 = lean_ctor_get(x_3, 3);
lean_inc(x_18);
lean_dec(x_3);
lean_inc(x_8);
lean_inc(x_18);
x_19 = lean_apply_2(x_8, x_12, x_18);
lean_inc(x_7);
x_20 = lean_apply_2(x_7, x_17, x_19);
x_21 = lean_ctor_get(x_2, 2);
lean_inc(x_21);
lean_inc(x_8);
lean_inc(x_21);
x_22 = lean_apply_2(x_8, x_21, x_10);
x_23 = lean_ctor_get(x_2, 3);
lean_inc(x_23);
lean_dec(x_2);
lean_inc(x_8);
lean_inc(x_23);
x_24 = lean_apply_2(x_8, x_23, x_13);
lean_inc(x_7);
x_25 = lean_apply_2(x_7, x_22, x_24);
lean_inc(x_8);
x_26 = lean_apply_2(x_8, x_21, x_16);
x_27 = lean_apply_2(x_8, x_23, x_18);
x_28 = lean_apply_2(x_7, x_26, x_27);
x_29 = lean_alloc_ctor(0, 4, 0);
lean_ctor_set(x_29, 0, x_15);
lean_ctor_set(x_29, 1, x_20);
lean_ctor_set(x_29, 2, x_25);
lean_ctor_set(x_29, 3, x_28);
return x_29;
}
}
LEAN_EXPORT lean_object* l_K22_Mat2_mul(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l_K22_Mat2_mul___rarg___boxed), 3, 0);
return x_2;
}
}
LEAN_EXPORT lean_object* l_K22_Mat2_mul___rarg___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = l_K22_Mat2_mul___rarg(x_1, x_2, x_3);
lean_dec(x_1);
return x_4;
}
}
LEAN_EXPORT lean_object* l_K22_Mat2_instMul___rarg(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l_K22_Mat2_mul___rarg___boxed), 3, 1);
lean_closure_set(x_2, 0, x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_K22_Mat2_instMul(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l_K22_Mat2_instMul___rarg), 1, 0);
return x_2;
}
}
LEAN_EXPORT lean_object* l_K22_Mat2_det___rarg(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; lean_object* x_10; lean_object* x_11; lean_object* x_12; 
x_3 = lean_ctor_get(x_1, 2);
lean_inc(x_3);
x_4 = l_CommRing_toNonUnitalCommRing___rarg(x_1);
lean_dec(x_1);
x_5 = lean_ctor_get(x_4, 1);
lean_inc(x_5);
lean_dec(x_4);
x_6 = lean_ctor_get(x_2, 0);
lean_inc(x_6);
x_7 = lean_ctor_get(x_2, 3);
lean_inc(x_7);
lean_inc(x_5);
x_8 = lean_apply_2(x_5, x_6, x_7);
x_9 = lean_ctor_get(x_2, 1);
lean_inc(x_9);
x_10 = lean_ctor_get(x_2, 2);
lean_inc(x_10);
lean_dec(x_2);
x_11 = lean_apply_2(x_5, x_9, x_10);
x_12 = lean_apply_2(x_3, x_8, x_11);
return x_12;
}
}
LEAN_EXPORT lean_object* l_K22_Mat2_det(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l_K22_Mat2_det___rarg), 2, 0);
return x_2;
}
}
LEAN_EXPORT lean_object* l_K22_burau___u03c3_u2081___rarg(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; 
x_3 = lean_ctor_get(x_1, 1);
lean_inc(x_3);
x_4 = lean_apply_1(x_3, x_2);
x_5 = lean_ctor_get(x_1, 0);
lean_inc(x_5);
lean_dec(x_1);
x_6 = lean_ctor_get(x_5, 1);
lean_inc(x_6);
x_7 = l_CommSemiring_toCommMonoidWithZero___rarg(x_5);
lean_dec(x_5);
x_8 = lean_ctor_get(x_7, 1);
lean_inc(x_8);
lean_dec(x_7);
lean_inc(x_6);
x_9 = lean_alloc_ctor(0, 4, 0);
lean_ctor_set(x_9, 0, x_4);
lean_ctor_set(x_9, 1, x_6);
lean_ctor_set(x_9, 2, x_8);
lean_ctor_set(x_9, 3, x_6);
return x_9;
}
}
LEAN_EXPORT lean_object* l_K22_burau___u03c3_u2081(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l_K22_burau___u03c3_u2081___rarg), 2, 0);
return x_2;
}
}
LEAN_EXPORT lean_object* l_K22_burau___u03c3_u2082___rarg(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; 
x_3 = lean_ctor_get(x_1, 0);
lean_inc(x_3);
x_4 = lean_ctor_get(x_3, 1);
lean_inc(x_4);
x_5 = l_CommSemiring_toCommMonoidWithZero___rarg(x_3);
lean_dec(x_3);
x_6 = lean_ctor_get(x_5, 1);
lean_inc(x_6);
lean_dec(x_5);
x_7 = lean_ctor_get(x_1, 1);
lean_inc(x_7);
lean_dec(x_1);
lean_inc(x_2);
x_8 = lean_apply_1(x_7, x_2);
x_9 = lean_alloc_ctor(0, 4, 0);
lean_ctor_set(x_9, 0, x_4);
lean_ctor_set(x_9, 1, x_6);
lean_ctor_set(x_9, 2, x_2);
lean_ctor_set(x_9, 3, x_8);
return x_9;
}
}
LEAN_EXPORT lean_object* l_K22_burau___u03c3_u2082(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l_K22_burau___u03c3_u2082___rarg), 2, 0);
return x_2;
}
}
LEAN_EXPORT lean_object* l_K22_det__burau___rarg(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; lean_object* x_5; 
x_4 = lean_apply_1(x_2, x_3);
x_5 = l_K22_Mat2_det___rarg(x_1, x_4);
return x_5;
}
}
LEAN_EXPORT lean_object* l_K22_det__burau(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l_K22_det__burau___rarg), 3, 0);
return x_2;
}
}
LEAN_EXPORT lean_object* l_List_mapTR_loop___at_K22_exponent__sum___spec__1(lean_object* x_1, lean_object* x_2) {
_start:
{
if (lean_obj_tag(x_1) == 0)
{
lean_object* x_3; 
x_3 = l_List_reverse___rarg(x_2);
return x_3;
}
else
{
uint8_t x_4; 
x_4 = !lean_is_exclusive(x_1);
if (x_4 == 0)
{
lean_object* x_5; lean_object* x_6; lean_object* x_7; 
x_5 = lean_ctor_get(x_1, 0);
x_6 = lean_ctor_get(x_1, 1);
x_7 = lean_ctor_get(x_5, 0);
lean_inc(x_7);
lean_dec(x_5);
lean_ctor_set(x_1, 1, x_2);
lean_ctor_set(x_1, 0, x_7);
{
lean_object* _tmp_0 = x_6;
lean_object* _tmp_1 = x_1;
x_1 = _tmp_0;
x_2 = _tmp_1;
}
goto _start;
}
else
{
lean_object* x_9; lean_object* x_10; lean_object* x_11; lean_object* x_12; 
x_9 = lean_ctor_get(x_1, 0);
x_10 = lean_ctor_get(x_1, 1);
lean_inc(x_10);
lean_inc(x_9);
lean_dec(x_1);
x_11 = lean_ctor_get(x_9, 0);
lean_inc(x_11);
lean_dec(x_9);
x_12 = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(x_12, 0, x_11);
lean_ctor_set(x_12, 1, x_2);
x_1 = x_10;
x_2 = x_12;
goto _start;
}
}
}
}
LEAN_EXPORT lean_object* l_List_foldl___at_K22_exponent__sum___spec__3(lean_object* x_1, lean_object* x_2) {
_start:
{
if (lean_obj_tag(x_2) == 0)
{
return x_1;
}
else
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; 
x_3 = lean_ctor_get(x_2, 0);
x_4 = lean_ctor_get(x_2, 1);
x_5 = lean_int_add(x_1, x_3);
lean_dec(x_1);
x_1 = x_5;
x_2 = x_4;
goto _start;
}
}
}
static lean_object* _init_l_List_sum___at_K22_exponent__sum___spec__2___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(0u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_List_sum___at_K22_exponent__sum___spec__2(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; 
x_2 = l_List_sum___at_K22_exponent__sum___spec__2___closed__1;
x_3 = l_List_foldl___at_K22_exponent__sum___spec__3(x_2, x_1);
lean_dec(x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* l_K22_exponent__sum(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; lean_object* x_4; 
x_2 = lean_box(0);
x_3 = l_List_mapTR_loop___at_K22_exponent__sum___spec__1(x_1, x_2);
x_4 = l_List_sum___at_K22_exponent__sum___spec__2(x_3);
return x_4;
}
}
LEAN_EXPORT lean_object* l_List_foldl___at_K22_exponent__sum___spec__3___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = l_List_foldl___at_K22_exponent__sum___spec__3(x_1, x_2);
lean_dec(x_2);
return x_3;
}
}
lean_object* initialize_Init(uint8_t builtin, lean_object*);
lean_object* initialize_Mathlib_Algebra_Ring_Basic(uint8_t builtin, lean_object*);
lean_object* initialize_Mathlib_Algebra_BigOperators_Group_List(uint8_t builtin, lean_object*);
lean_object* initialize_Mathlib_Data_List_Basic(uint8_t builtin, lean_object*);
lean_object* initialize_Mathlib_Data_Finset_Basic(uint8_t builtin, lean_object*);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_K22_Syntax(uint8_t builtin, lean_object* w) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Mathlib_Algebra_Ring_Basic(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Mathlib_Algebra_BigOperators_Group_List(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Mathlib_Data_List_Basic(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Mathlib_Data_Finset_Basic(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
l_List_sum___at_K22_exponent__sum___spec__2___closed__1 = _init_l_List_sum___at_K22_exponent__sum___spec__2___closed__1();
lean_mark_persistent(l_List_sum___at_K22_exponent__sum___spec__2___closed__1);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
