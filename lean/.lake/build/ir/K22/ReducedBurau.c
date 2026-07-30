// Lean compiler output
// Module: K22.ReducedBurau
// Imports: Init K22.Syntax
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
static lean_object* l_K22_writheExponent___closed__2;
LEAN_EXPORT lean_object* l_K22_reducedBurauDet___rarg___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* l_K22_burau___u03c3_u2081___rarg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_burauGen___rarg___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* l_NonUnitalNonAssocSemiring_toDistrib___rarg(lean_object*);
lean_object* l_K22_Mat2_det___rarg(lean_object*, lean_object*);
lean_object* l_NonUnitalNonAssocCommRing_toNonUnitalNonAssocCommSemiring___rarg(lean_object*);
lean_object* lean_nat_to_int(lean_object*);
lean_object* l_CommSemiring_toCommMonoidWithZero___rarg(lean_object*);
LEAN_EXPORT lean_object* l_K22_burauTrace(lean_object*);
LEAN_EXPORT lean_object* l_K22_burauGen___rarg(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_burauGen(lean_object*);
LEAN_EXPORT lean_object* l_K22_writheExponent(lean_object*);
LEAN_EXPORT lean_object* l_List_foldl___at_K22_reducedBurauWord___spec__1___rarg___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_matId___rarg___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_reducedBurauWord(lean_object*);
LEAN_EXPORT lean_object* l_K22_burauMul___rarg___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_reducedBurauWord___rarg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_burauMul___rarg(lean_object*, lean_object*, lean_object*);
lean_object* lean_int_mul(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_matId(lean_object*);
uint8_t lean_nat_dec_eq(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_reducedBurauDet(lean_object*);
LEAN_EXPORT lean_object* l_K22_burauTrace___rarg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_List_foldl___at_K22_reducedBurauWord___spec__1(lean_object*);
LEAN_EXPORT lean_object* l_K22_burauTrace___rarg___boxed(lean_object*, lean_object*);
static lean_object* l_K22_writheExponent___closed__1;
LEAN_EXPORT lean_object* l_K22_matId___rarg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_List_foldl___at_K22_reducedBurauWord___spec__1___rarg(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* l_K22_burau___u03c3_u2082___rarg(lean_object*, lean_object*);
lean_object* l_K22_exponent__sum(lean_object*);
LEAN_EXPORT lean_object* l_K22_burauMul(lean_object*);
lean_object* lean_int_neg(lean_object*);
uint8_t lean_nat_dec_le(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_reducedBurauDet___rarg(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lean_nat_add(lean_object*, lean_object*);
lean_object* l_K22_Mat2_mul___rarg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_matId___rarg(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; lean_object* x_4; 
x_3 = lean_ctor_get(x_1, 0);
x_4 = lean_ctor_get(x_3, 1);
lean_inc(x_4);
return x_4;
}
}
LEAN_EXPORT lean_object* l_K22_matId(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l_K22_matId___rarg___boxed), 2, 0);
return x_2;
}
}
LEAN_EXPORT lean_object* l_K22_matId___rarg___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = l_K22_matId___rarg(x_1, x_2);
lean_dec(x_2);
lean_dec(x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* l_K22_burauGen___rarg(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_object* x_5; uint8_t x_6; 
x_5 = lean_unsigned_to_nat(2u);
x_6 = lean_nat_dec_le(x_2, x_5);
if (x_6 == 0)
{
lean_object* x_7; uint8_t x_8; 
x_7 = lean_unsigned_to_nat(1u);
x_8 = lean_nat_dec_eq(x_3, x_7);
if (x_8 == 0)
{
uint8_t x_9; 
x_9 = lean_nat_dec_eq(x_3, x_5);
if (x_9 == 0)
{
lean_object* x_10; lean_object* x_11; lean_object* x_12; lean_object* x_13; lean_object* x_14; 
lean_dec(x_4);
x_10 = lean_ctor_get(x_1, 0);
lean_inc(x_10);
lean_dec(x_1);
x_11 = lean_ctor_get(x_10, 1);
lean_inc(x_11);
x_12 = l_CommSemiring_toCommMonoidWithZero___rarg(x_10);
lean_dec(x_10);
x_13 = lean_ctor_get(x_12, 1);
lean_inc(x_13);
lean_dec(x_12);
lean_inc(x_13);
lean_inc(x_11);
x_14 = lean_alloc_ctor(0, 4, 0);
lean_ctor_set(x_14, 0, x_11);
lean_ctor_set(x_14, 1, x_13);
lean_ctor_set(x_14, 2, x_13);
lean_ctor_set(x_14, 3, x_11);
return x_14;
}
else
{
lean_object* x_15; 
x_15 = l_K22_burau___u03c3_u2082___rarg(x_1, x_4);
return x_15;
}
}
else
{
lean_object* x_16; 
x_16 = l_K22_burau___u03c3_u2081___rarg(x_1, x_4);
return x_16;
}
}
else
{
lean_object* x_17; lean_object* x_18; lean_object* x_19; lean_object* x_20; lean_object* x_21; lean_object* x_22; 
x_17 = lean_ctor_get(x_1, 1);
lean_inc(x_17);
x_18 = lean_apply_1(x_17, x_4);
x_19 = lean_ctor_get(x_1, 0);
lean_inc(x_19);
lean_dec(x_1);
x_20 = l_CommSemiring_toCommMonoidWithZero___rarg(x_19);
lean_dec(x_19);
x_21 = lean_ctor_get(x_20, 1);
lean_inc(x_21);
lean_dec(x_20);
lean_inc_n(x_21, 2);
x_22 = lean_alloc_ctor(0, 4, 0);
lean_ctor_set(x_22, 0, x_18);
lean_ctor_set(x_22, 1, x_21);
lean_ctor_set(x_22, 2, x_21);
lean_ctor_set(x_22, 3, x_21);
return x_22;
}
}
}
LEAN_EXPORT lean_object* l_K22_burauGen(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l_K22_burauGen___rarg___boxed), 4, 0);
return x_2;
}
}
LEAN_EXPORT lean_object* l_K22_burauGen___rarg___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_object* x_5; 
x_5 = l_K22_burauGen___rarg(x_1, x_2, x_3, x_4);
lean_dec(x_3);
lean_dec(x_2);
return x_5;
}
}
LEAN_EXPORT lean_object* l_K22_burauMul___rarg(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = l_K22_Mat2_mul___rarg(x_1, x_2, x_3);
return x_4;
}
}
LEAN_EXPORT lean_object* l_K22_burauMul(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l_K22_burauMul___rarg___boxed), 3, 0);
return x_2;
}
}
LEAN_EXPORT lean_object* l_K22_burauMul___rarg___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = l_K22_burauMul___rarg(x_1, x_2, x_3);
lean_dec(x_1);
return x_4;
}
}
LEAN_EXPORT lean_object* l_List_foldl___at_K22_reducedBurauWord___spec__1___rarg(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
if (lean_obj_tag(x_4) == 0)
{
lean_dec(x_2);
lean_dec(x_1);
return x_3;
}
else
{
lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; lean_object* x_10; lean_object* x_11; lean_object* x_12; 
x_5 = lean_ctor_get(x_4, 0);
x_6 = lean_ctor_get(x_4, 1);
x_7 = lean_ctor_get(x_5, 1);
x_8 = lean_unsigned_to_nat(1u);
x_9 = lean_nat_add(x_7, x_8);
x_10 = lean_unsigned_to_nat(2u);
lean_inc(x_2);
lean_inc(x_1);
x_11 = l_K22_burauGen___rarg(x_1, x_10, x_9, x_2);
lean_dec(x_9);
x_12 = l_K22_Mat2_mul___rarg(x_1, x_3, x_11);
x_3 = x_12;
x_4 = x_6;
goto _start;
}
}
}
LEAN_EXPORT lean_object* l_List_foldl___at_K22_reducedBurauWord___spec__1(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l_List_foldl___at_K22_reducedBurauWord___spec__1___rarg___boxed), 4, 0);
return x_2;
}
}
LEAN_EXPORT lean_object* l_K22_reducedBurauWord___rarg(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; 
x_4 = lean_ctor_get(x_1, 0);
lean_inc(x_4);
x_5 = lean_ctor_get(x_4, 1);
lean_inc(x_5);
x_6 = l_CommSemiring_toCommMonoidWithZero___rarg(x_4);
lean_dec(x_4);
x_7 = lean_ctor_get(x_6, 1);
lean_inc(x_7);
lean_dec(x_6);
lean_inc(x_7);
lean_inc(x_5);
x_8 = lean_alloc_ctor(0, 4, 0);
lean_ctor_set(x_8, 0, x_5);
lean_ctor_set(x_8, 1, x_7);
lean_ctor_set(x_8, 2, x_7);
lean_ctor_set(x_8, 3, x_5);
x_9 = l_List_foldl___at_K22_reducedBurauWord___spec__1___rarg(x_1, x_2, x_8, x_3);
lean_dec(x_3);
return x_9;
}
}
LEAN_EXPORT lean_object* l_K22_reducedBurauWord(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l_K22_reducedBurauWord___rarg), 3, 0);
return x_2;
}
}
LEAN_EXPORT lean_object* l_List_foldl___at_K22_reducedBurauWord___spec__1___rarg___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_object* x_5; 
x_5 = l_List_foldl___at_K22_reducedBurauWord___spec__1___rarg(x_1, x_2, x_3, x_4);
lean_dec(x_4);
return x_5;
}
}
LEAN_EXPORT lean_object* l_K22_burauTrace___rarg(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; 
x_3 = l_CommRing_toNonUnitalCommRing___rarg(x_1);
x_4 = l_NonUnitalNonAssocCommRing_toNonUnitalNonAssocCommSemiring___rarg(x_3);
lean_dec(x_3);
x_5 = l_NonUnitalNonAssocSemiring_toDistrib___rarg(x_4);
lean_dec(x_4);
x_6 = lean_ctor_get(x_5, 1);
lean_inc(x_6);
lean_dec(x_5);
x_7 = lean_ctor_get(x_2, 0);
lean_inc(x_7);
x_8 = lean_ctor_get(x_2, 3);
lean_inc(x_8);
lean_dec(x_2);
x_9 = lean_apply_2(x_6, x_7, x_8);
return x_9;
}
}
LEAN_EXPORT lean_object* l_K22_burauTrace(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l_K22_burauTrace___rarg___boxed), 2, 0);
return x_2;
}
}
LEAN_EXPORT lean_object* l_K22_burauTrace___rarg___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = l_K22_burauTrace___rarg(x_1, x_2);
lean_dec(x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* l_K22_reducedBurauDet___rarg(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_object* x_5; uint8_t x_6; 
x_5 = lean_unsigned_to_nat(2u);
x_6 = lean_nat_dec_le(x_2, x_5);
if (x_6 == 0)
{
lean_object* x_7; lean_object* x_8; 
lean_inc(x_1);
x_7 = l_K22_reducedBurauWord___rarg(x_1, x_3, x_4);
x_8 = l_K22_Mat2_det___rarg(x_1, x_7);
return x_8;
}
else
{
lean_object* x_9; lean_object* x_10; 
x_9 = l_K22_reducedBurauWord___rarg(x_1, x_3, x_4);
x_10 = lean_ctor_get(x_9, 0);
lean_inc(x_10);
lean_dec(x_9);
return x_10;
}
}
}
LEAN_EXPORT lean_object* l_K22_reducedBurauDet(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l_K22_reducedBurauDet___rarg___boxed), 4, 0);
return x_2;
}
}
LEAN_EXPORT lean_object* l_K22_reducedBurauDet___rarg___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_object* x_5; 
x_5 = l_K22_reducedBurauDet___rarg(x_1, x_2, x_3, x_4);
lean_dec(x_2);
return x_5;
}
}
static lean_object* _init_l_K22_writheExponent___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(3u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_l_K22_writheExponent___closed__2() {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = l_K22_writheExponent___closed__1;
x_2 = lean_int_neg(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_K22_writheExponent(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; lean_object* x_4; 
x_2 = l_K22_exponent__sum(x_1);
x_3 = l_K22_writheExponent___closed__2;
x_4 = lean_int_mul(x_3, x_2);
lean_dec(x_2);
return x_4;
}
}
lean_object* initialize_Init(uint8_t builtin, lean_object*);
lean_object* initialize_K22_Syntax(uint8_t builtin, lean_object*);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_K22_ReducedBurau(uint8_t builtin, lean_object* w) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_K22_Syntax(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
l_K22_writheExponent___closed__1 = _init_l_K22_writheExponent___closed__1();
lean_mark_persistent(l_K22_writheExponent___closed__1);
l_K22_writheExponent___closed__2 = _init_l_K22_writheExponent___closed__2();
lean_mark_persistent(l_K22_writheExponent___closed__2);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
