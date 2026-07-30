// Lean compiler output
// Module: K22.MOG.ParityLiftRank
// Imports: Init Mathlib.Data.Nat.Bitwise Mathlib.Data.List.Range
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
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_evenSumKernelDimension;
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_listSwap(lean_object*, lean_object*, lean_object*);
static lean_object* l_K22_MOG_ParityLiftRank_col__parity__sum__is__all__ones___nativeDecide__1___closed__1;
static lean_object* l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__1;
static lean_object* l_K22_MOG_ParityLiftRank_colParityMask___closed__1;
lean_object* lean_mk_empty_array_with_capacity(lean_object*);
LEAN_EXPORT lean_object* l_List_mapTR_loop___at_K22_MOG_ParityLiftRank_parityMasks___spec__2(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_parityMatrixRank;
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_pivotStep___spec__1___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_gf2Rank___spec__1___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_row__col__parity__global__sum___nativeDecide__1___spec__2(lean_object*, lean_object*);
static lean_object* l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__1;
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_colParityMask(lean_object*);
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_row__col__parity__global__sum___nativeDecide__1___spec__2___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_pivotStep(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_listSet(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_pivotStep___spec__1(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* l_List_getD___rarg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_rowParityMask(lean_object*);
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_row__col__parity__global__sum___nativeDecide__1___spec__1___boxed(lean_object*, lean_object*);
lean_object* lean_nat_shiftr(lean_object*, lean_object*);
static uint8_t l_K22_MOG_ParityLiftRank_parity__matrix__rank__eq__nine___nativeDecide__1___closed__1;
static lean_object* l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__3;
static lean_object* l_K22_MOG_ParityLiftRank_parityMatrixRank___closed__1;
LEAN_EXPORT uint8_t l_K22_MOG_ParityLiftRank_row__col__parity__global__sum___nativeDecide__1;
lean_object* l_List_appendTR___rarg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_pivotStep_find___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint8_t l_K22_MOG_ParityLiftRank_col__parity__sum__is__all__ones___nativeDecide__1;
static uint8_t l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__4;
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_colParityMask___boxed(lean_object*);
lean_object* l_List_range(lean_object*);
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_rowParityMask___boxed(lean_object*);
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_listSet___boxed(lean_object*, lean_object*, lean_object*);
static lean_object* l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__3;
lean_object* l_List_lengthTRAux___rarg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_parityMasks;
LEAN_EXPORT uint8_t l_K22_MOG_ParityLiftRank_parity__matrix__rank__eq__nine___nativeDecide__1;
static uint8_t l_K22_MOG_ParityLiftRank_col__parity__sum__is__all__ones___nativeDecide__1___closed__2;
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_rowParityMask___spec__1(lean_object*, lean_object*, lean_object*);
static uint8_t l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__4;
LEAN_EXPORT uint8_t l_K22_MOG_ParityLiftRank_bitAt(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_gf2Rank(lean_object*);
lean_object* lean_nat_lxor(lean_object*, lean_object*);
uint8_t lean_nat_dec_eq(lean_object*, lean_object*);
lean_object* l_List_takeTR_go___rarg(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lean_nat_mod(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_colParityMask___spec__1(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_rowParityMask___spec__1___boxed(lean_object*, lean_object*, lean_object*);
static lean_object* l_K22_MOG_ParityLiftRank_gf2Rank___closed__1;
lean_object* lean_nat_shiftl(lean_object*, lean_object*);
lean_object* lean_nat_sub(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_pivotStep___boxed(lean_object*, lean_object*, lean_object*);
lean_object* lean_nat_mul(lean_object*, lean_object*);
static lean_object* l_K22_MOG_ParityLiftRank_rowParityMask___closed__1;
lean_object* l_List_reverse___rarg(lean_object*);
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_pivotStep_find(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint8_t l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1;
static lean_object* l_K22_MOG_ParityLiftRank_evenSumKernelDimension___closed__1;
LEAN_EXPORT uint8_t l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1;
LEAN_EXPORT lean_object* l_List_mapTR_loop___at_K22_MOG_ParityLiftRank_parityMasks___spec__1(lean_object*, lean_object*);
static lean_object* l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__2;
uint8_t lean_nat_dec_le(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_row__col__parity__global__sum___nativeDecide__1___spec__1(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_bitAt___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_gf2Rank___spec__1(lean_object*, lean_object*);
lean_object* lean_nat_add(lean_object*, lean_object*);
static lean_object* l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__2;
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_colParityMask___spec__1___boxed(lean_object*, lean_object*, lean_object*);
lean_object* lean_nat_lor(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_rowParityMask___spec__1(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
if (lean_obj_tag(x_3) == 0)
{
return x_2;
}
else
{
lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; lean_object* x_10; lean_object* x_11; 
x_4 = lean_ctor_get(x_3, 0);
x_5 = lean_ctor_get(x_3, 1);
x_6 = lean_unsigned_to_nat(6u);
x_7 = lean_nat_mul(x_6, x_1);
x_8 = lean_nat_add(x_7, x_4);
lean_dec(x_7);
x_9 = lean_unsigned_to_nat(1u);
x_10 = lean_nat_shiftl(x_9, x_8);
lean_dec(x_8);
x_11 = lean_nat_lor(x_2, x_10);
lean_dec(x_10);
lean_dec(x_2);
x_2 = x_11;
x_3 = x_5;
goto _start;
}
}
}
static lean_object* _init_l_K22_MOG_ParityLiftRank_rowParityMask___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(6u);
x_2 = l_List_range(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_rowParityMask(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; lean_object* x_4; 
x_2 = lean_unsigned_to_nat(0u);
x_3 = l_K22_MOG_ParityLiftRank_rowParityMask___closed__1;
x_4 = l_List_foldl___at_K22_MOG_ParityLiftRank_rowParityMask___spec__1(x_1, x_2, x_3);
return x_4;
}
}
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_rowParityMask___spec__1___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = l_List_foldl___at_K22_MOG_ParityLiftRank_rowParityMask___spec__1(x_1, x_2, x_3);
lean_dec(x_3);
lean_dec(x_1);
return x_4;
}
}
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_rowParityMask___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = l_K22_MOG_ParityLiftRank_rowParityMask(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_colParityMask___spec__1(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
if (lean_obj_tag(x_3) == 0)
{
return x_2;
}
else
{
lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; lean_object* x_10; lean_object* x_11; 
x_4 = lean_ctor_get(x_3, 0);
x_5 = lean_ctor_get(x_3, 1);
x_6 = lean_unsigned_to_nat(6u);
x_7 = lean_nat_mul(x_6, x_4);
x_8 = lean_nat_add(x_7, x_1);
lean_dec(x_7);
x_9 = lean_unsigned_to_nat(1u);
x_10 = lean_nat_shiftl(x_9, x_8);
lean_dec(x_8);
x_11 = lean_nat_lor(x_2, x_10);
lean_dec(x_10);
lean_dec(x_2);
x_2 = x_11;
x_3 = x_5;
goto _start;
}
}
}
static lean_object* _init_l_K22_MOG_ParityLiftRank_colParityMask___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(4u);
x_2 = l_List_range(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_colParityMask(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; lean_object* x_4; 
x_2 = lean_unsigned_to_nat(0u);
x_3 = l_K22_MOG_ParityLiftRank_colParityMask___closed__1;
x_4 = l_List_foldl___at_K22_MOG_ParityLiftRank_colParityMask___spec__1(x_1, x_2, x_3);
return x_4;
}
}
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_colParityMask___spec__1___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = l_List_foldl___at_K22_MOG_ParityLiftRank_colParityMask___spec__1(x_1, x_2, x_3);
lean_dec(x_3);
lean_dec(x_1);
return x_4;
}
}
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_colParityMask___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = l_K22_MOG_ParityLiftRank_colParityMask(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_List_mapTR_loop___at_K22_MOG_ParityLiftRank_parityMasks___spec__1(lean_object* x_1, lean_object* x_2) {
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
x_7 = l_K22_MOG_ParityLiftRank_rowParityMask(x_5);
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
x_11 = l_K22_MOG_ParityLiftRank_rowParityMask(x_9);
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
LEAN_EXPORT lean_object* l_List_mapTR_loop___at_K22_MOG_ParityLiftRank_parityMasks___spec__2(lean_object* x_1, lean_object* x_2) {
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
x_7 = l_K22_MOG_ParityLiftRank_colParityMask(x_5);
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
x_11 = l_K22_MOG_ParityLiftRank_colParityMask(x_9);
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
static lean_object* _init_l_K22_MOG_ParityLiftRank_parityMasks() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; 
x_1 = lean_box(0);
x_2 = l_K22_MOG_ParityLiftRank_colParityMask___closed__1;
x_3 = l_List_mapTR_loop___at_K22_MOG_ParityLiftRank_parityMasks___spec__1(x_2, x_1);
x_4 = l_K22_MOG_ParityLiftRank_rowParityMask___closed__1;
x_5 = l_List_mapTR_loop___at_K22_MOG_ParityLiftRank_parityMasks___spec__2(x_4, x_1);
x_6 = l_List_appendTR___rarg(x_3, x_5);
return x_6;
}
}
LEAN_EXPORT uint8_t l_K22_MOG_ParityLiftRank_bitAt(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; uint8_t x_7; 
x_3 = lean_nat_shiftr(x_1, x_2);
x_4 = lean_unsigned_to_nat(2u);
x_5 = lean_nat_mod(x_3, x_4);
lean_dec(x_3);
x_6 = lean_unsigned_to_nat(1u);
x_7 = lean_nat_dec_eq(x_5, x_6);
lean_dec(x_5);
return x_7;
}
}
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_bitAt___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
uint8_t x_3; lean_object* x_4; 
x_3 = l_K22_MOG_ParityLiftRank_bitAt(x_1, x_2);
lean_dec(x_2);
lean_dec(x_1);
x_4 = lean_box(x_3);
return x_4;
}
}
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_listSet(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
if (lean_obj_tag(x_1) == 0)
{
lean_object* x_4; 
lean_dec(x_3);
x_4 = lean_box(0);
return x_4;
}
else
{
uint8_t x_5; 
x_5 = !lean_is_exclusive(x_1);
if (x_5 == 0)
{
lean_object* x_6; lean_object* x_7; lean_object* x_8; uint8_t x_9; 
x_6 = lean_ctor_get(x_1, 0);
x_7 = lean_ctor_get(x_1, 1);
x_8 = lean_unsigned_to_nat(0u);
x_9 = lean_nat_dec_eq(x_2, x_8);
if (x_9 == 0)
{
lean_object* x_10; lean_object* x_11; lean_object* x_12; 
x_10 = lean_unsigned_to_nat(1u);
x_11 = lean_nat_sub(x_2, x_10);
x_12 = l_K22_MOG_ParityLiftRank_listSet(x_7, x_11, x_3);
lean_dec(x_11);
lean_ctor_set(x_1, 1, x_12);
return x_1;
}
else
{
lean_dec(x_6);
lean_ctor_set(x_1, 0, x_3);
return x_1;
}
}
else
{
lean_object* x_13; lean_object* x_14; lean_object* x_15; uint8_t x_16; 
x_13 = lean_ctor_get(x_1, 0);
x_14 = lean_ctor_get(x_1, 1);
lean_inc(x_14);
lean_inc(x_13);
lean_dec(x_1);
x_15 = lean_unsigned_to_nat(0u);
x_16 = lean_nat_dec_eq(x_2, x_15);
if (x_16 == 0)
{
lean_object* x_17; lean_object* x_18; lean_object* x_19; lean_object* x_20; 
x_17 = lean_unsigned_to_nat(1u);
x_18 = lean_nat_sub(x_2, x_17);
x_19 = l_K22_MOG_ParityLiftRank_listSet(x_14, x_18, x_3);
lean_dec(x_18);
x_20 = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(x_20, 0, x_13);
lean_ctor_set(x_20, 1, x_19);
return x_20;
}
else
{
lean_object* x_21; 
lean_dec(x_13);
x_21 = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(x_21, 0, x_3);
lean_ctor_set(x_21, 1, x_14);
return x_21;
}
}
}
}
}
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_listSet___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = l_K22_MOG_ParityLiftRank_listSet(x_1, x_2, x_3);
lean_dec(x_2);
return x_4;
}
}
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_listSwap(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
uint8_t x_4; 
x_4 = lean_nat_dec_eq(x_2, x_3);
if (x_4 == 0)
{
lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; 
x_5 = lean_unsigned_to_nat(0u);
lean_inc(x_2);
x_6 = l_List_getD___rarg(x_1, x_2, x_5);
lean_inc(x_3);
x_7 = l_List_getD___rarg(x_1, x_3, x_5);
x_8 = l_K22_MOG_ParityLiftRank_listSet(x_1, x_2, x_7);
lean_dec(x_2);
x_9 = l_K22_MOG_ParityLiftRank_listSet(x_8, x_3, x_6);
lean_dec(x_3);
return x_9;
}
else
{
lean_dec(x_3);
lean_dec(x_2);
return x_1;
}
}
}
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_pivotStep_find(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
uint8_t x_5; 
x_5 = lean_nat_dec_le(x_3, x_4);
if (x_5 == 0)
{
lean_object* x_6; lean_object* x_7; uint8_t x_8; 
x_6 = lean_unsigned_to_nat(0u);
lean_inc(x_4);
x_7 = l_List_getD___rarg(x_1, x_4, x_6);
x_8 = l_K22_MOG_ParityLiftRank_bitAt(x_7, x_2);
lean_dec(x_7);
if (x_8 == 0)
{
lean_object* x_9; lean_object* x_10; 
x_9 = lean_unsigned_to_nat(1u);
x_10 = lean_nat_add(x_4, x_9);
lean_dec(x_4);
x_4 = x_10;
goto _start;
}
else
{
lean_object* x_12; 
x_12 = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(x_12, 0, x_4);
return x_12;
}
}
else
{
lean_object* x_13; 
lean_dec(x_4);
x_13 = lean_box(0);
return x_13;
}
}
}
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_pivotStep_find___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_object* x_5; 
x_5 = l_K22_MOG_ParityLiftRank_pivotStep_find(x_1, x_2, x_3, x_4);
lean_dec(x_3);
lean_dec(x_2);
lean_dec(x_1);
return x_5;
}
}
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_pivotStep___spec__1(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5) {
_start:
{
if (lean_obj_tag(x_5) == 0)
{
return x_4;
}
else
{
lean_object* x_6; lean_object* x_7; uint8_t x_8; 
x_6 = lean_ctor_get(x_5, 0);
lean_inc(x_6);
x_7 = lean_ctor_get(x_5, 1);
lean_inc(x_7);
lean_dec(x_5);
x_8 = lean_nat_dec_eq(x_6, x_2);
if (x_8 == 0)
{
lean_object* x_9; lean_object* x_10; uint8_t x_11; 
x_9 = lean_unsigned_to_nat(0u);
lean_inc(x_6);
x_10 = l_List_getD___rarg(x_4, x_6, x_9);
x_11 = l_K22_MOG_ParityLiftRank_bitAt(x_10, x_1);
if (x_11 == 0)
{
lean_dec(x_10);
lean_dec(x_6);
x_5 = x_7;
goto _start;
}
else
{
lean_object* x_13; lean_object* x_14; 
x_13 = lean_nat_lxor(x_10, x_3);
lean_dec(x_10);
x_14 = l_K22_MOG_ParityLiftRank_listSet(x_4, x_6, x_13);
lean_dec(x_6);
x_4 = x_14;
x_5 = x_7;
goto _start;
}
}
else
{
lean_dec(x_6);
x_5 = x_7;
goto _start;
}
}
}
}
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_pivotStep(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; lean_object* x_5; lean_object* x_6; 
x_4 = lean_unsigned_to_nat(0u);
x_5 = l_List_lengthTRAux___rarg(x_1, x_4);
lean_inc(x_3);
x_6 = l_K22_MOG_ParityLiftRank_pivotStep_find(x_1, x_2, x_5, x_3);
if (lean_obj_tag(x_6) == 0)
{
lean_object* x_7; 
lean_dec(x_5);
x_7 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_7, 0, x_1);
lean_ctor_set(x_7, 1, x_3);
return x_7;
}
else
{
lean_object* x_8; lean_object* x_9; lean_object* x_10; lean_object* x_11; lean_object* x_12; lean_object* x_13; lean_object* x_14; lean_object* x_15; 
x_8 = lean_ctor_get(x_6, 0);
lean_inc(x_8);
lean_dec(x_6);
lean_inc(x_3);
x_9 = l_K22_MOG_ParityLiftRank_listSwap(x_1, x_3, x_8);
lean_inc(x_3);
x_10 = l_List_getD___rarg(x_9, x_3, x_4);
x_11 = l_List_range(x_5);
x_12 = l_List_foldl___at_K22_MOG_ParityLiftRank_pivotStep___spec__1(x_2, x_3, x_10, x_9, x_11);
lean_dec(x_10);
x_13 = lean_unsigned_to_nat(1u);
x_14 = lean_nat_add(x_3, x_13);
lean_dec(x_3);
x_15 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_15, 0, x_12);
lean_ctor_set(x_15, 1, x_14);
return x_15;
}
}
}
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_pivotStep___spec__1___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5) {
_start:
{
lean_object* x_6; 
x_6 = l_List_foldl___at_K22_MOG_ParityLiftRank_pivotStep___spec__1(x_1, x_2, x_3, x_4, x_5);
lean_dec(x_3);
lean_dec(x_2);
lean_dec(x_1);
return x_6;
}
}
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_pivotStep___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = l_K22_MOG_ParityLiftRank_pivotStep(x_1, x_2, x_3);
lean_dec(x_2);
return x_4;
}
}
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_gf2Rank___spec__1(lean_object* x_1, lean_object* x_2) {
_start:
{
if (lean_obj_tag(x_2) == 0)
{
return x_1;
}
else
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; 
x_3 = lean_ctor_get(x_2, 0);
x_4 = lean_ctor_get(x_2, 1);
x_5 = lean_ctor_get(x_1, 0);
lean_inc(x_5);
x_6 = lean_ctor_get(x_1, 1);
lean_inc(x_6);
lean_dec(x_1);
x_7 = l_K22_MOG_ParityLiftRank_pivotStep(x_5, x_3, x_6);
x_1 = x_7;
x_2 = x_4;
goto _start;
}
}
}
static lean_object* _init_l_K22_MOG_ParityLiftRank_gf2Rank___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(24u);
x_2 = l_List_range(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_K22_MOG_ParityLiftRank_gf2Rank(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; 
x_2 = lean_unsigned_to_nat(0u);
x_3 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_3, 0, x_1);
lean_ctor_set(x_3, 1, x_2);
x_4 = l_K22_MOG_ParityLiftRank_gf2Rank___closed__1;
x_5 = l_List_foldl___at_K22_MOG_ParityLiftRank_gf2Rank___spec__1(x_3, x_4);
x_6 = lean_ctor_get(x_5, 1);
lean_inc(x_6);
lean_dec(x_5);
return x_6;
}
}
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_gf2Rank___spec__1___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = l_List_foldl___at_K22_MOG_ParityLiftRank_gf2Rank___spec__1(x_1, x_2);
lean_dec(x_2);
return x_3;
}
}
static lean_object* _init_l_K22_MOG_ParityLiftRank_parityMatrixRank___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = l_K22_MOG_ParityLiftRank_parityMasks;
x_2 = l_K22_MOG_ParityLiftRank_gf2Rank(x_1);
return x_2;
}
}
static lean_object* _init_l_K22_MOG_ParityLiftRank_parityMatrixRank() {
_start:
{
lean_object* x_1; 
x_1 = l_K22_MOG_ParityLiftRank_parityMatrixRank___closed__1;
return x_1;
}
}
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_row__col__parity__global__sum___nativeDecide__1___spec__1(lean_object* x_1, lean_object* x_2) {
_start:
{
if (lean_obj_tag(x_2) == 0)
{
return x_1;
}
else
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; 
x_3 = lean_ctor_get(x_2, 0);
x_4 = lean_ctor_get(x_2, 1);
x_5 = l_K22_MOG_ParityLiftRank_rowParityMask(x_3);
x_6 = lean_nat_lxor(x_1, x_5);
lean_dec(x_5);
lean_dec(x_1);
x_1 = x_6;
x_2 = x_4;
goto _start;
}
}
}
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_row__col__parity__global__sum___nativeDecide__1___spec__2(lean_object* x_1, lean_object* x_2) {
_start:
{
if (lean_obj_tag(x_2) == 0)
{
return x_1;
}
else
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; 
x_3 = lean_ctor_get(x_2, 0);
x_4 = lean_ctor_get(x_2, 1);
x_5 = l_K22_MOG_ParityLiftRank_colParityMask(x_3);
x_6 = lean_nat_lxor(x_1, x_5);
lean_dec(x_5);
lean_dec(x_1);
x_1 = x_6;
x_2 = x_4;
goto _start;
}
}
}
static uint8_t _init_l_K22_MOG_ParityLiftRank_row__col__parity__global__sum___nativeDecide__1() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; lean_object* x_4; lean_object* x_5; uint8_t x_6; 
x_1 = lean_unsigned_to_nat(0u);
x_2 = l_K22_MOG_ParityLiftRank_colParityMask___closed__1;
x_3 = l_List_foldl___at_K22_MOG_ParityLiftRank_row__col__parity__global__sum___nativeDecide__1___spec__1(x_1, x_2);
x_4 = l_K22_MOG_ParityLiftRank_rowParityMask___closed__1;
x_5 = l_List_foldl___at_K22_MOG_ParityLiftRank_row__col__parity__global__sum___nativeDecide__1___spec__2(x_1, x_4);
x_6 = lean_nat_dec_eq(x_3, x_5);
lean_dec(x_5);
lean_dec(x_3);
return x_6;
}
}
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_row__col__parity__global__sum___nativeDecide__1___spec__1___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = l_List_foldl___at_K22_MOG_ParityLiftRank_row__col__parity__global__sum___nativeDecide__1___spec__1(x_1, x_2);
lean_dec(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* l_List_foldl___at_K22_MOG_ParityLiftRank_row__col__parity__global__sum___nativeDecide__1___spec__2___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = l_List_foldl___at_K22_MOG_ParityLiftRank_row__col__parity__global__sum___nativeDecide__1___spec__2(x_1, x_2);
lean_dec(x_2);
return x_3;
}
}
static lean_object* _init_l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = lean_unsigned_to_nat(0u);
x_2 = l_K22_MOG_ParityLiftRank_colParityMask___closed__1;
x_3 = l_List_foldl___at_K22_MOG_ParityLiftRank_row__col__parity__global__sum___nativeDecide__1___spec__1(x_1, x_2);
return x_3;
}
}
static lean_object* _init_l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__2() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = lean_unsigned_to_nat(1u);
x_2 = lean_unsigned_to_nat(24u);
x_3 = lean_nat_shiftl(x_1, x_2);
return x_3;
}
}
static lean_object* _init_l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__3() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__2;
x_2 = lean_unsigned_to_nat(1u);
x_3 = lean_nat_sub(x_1, x_2);
return x_3;
}
}
static uint8_t _init_l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__4() {
_start:
{
lean_object* x_1; lean_object* x_2; uint8_t x_3; 
x_1 = l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__1;
x_2 = l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__3;
x_3 = lean_nat_dec_eq(x_1, x_2);
return x_3;
}
}
static uint8_t _init_l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1() {
_start:
{
uint8_t x_1; 
x_1 = l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__4;
return x_1;
}
}
static lean_object* _init_l_K22_MOG_ParityLiftRank_col__parity__sum__is__all__ones___nativeDecide__1___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = lean_unsigned_to_nat(0u);
x_2 = l_K22_MOG_ParityLiftRank_rowParityMask___closed__1;
x_3 = l_List_foldl___at_K22_MOG_ParityLiftRank_row__col__parity__global__sum___nativeDecide__1___spec__2(x_1, x_2);
return x_3;
}
}
static uint8_t _init_l_K22_MOG_ParityLiftRank_col__parity__sum__is__all__ones___nativeDecide__1___closed__2() {
_start:
{
lean_object* x_1; lean_object* x_2; uint8_t x_3; 
x_1 = l_K22_MOG_ParityLiftRank_col__parity__sum__is__all__ones___nativeDecide__1___closed__1;
x_2 = l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__3;
x_3 = lean_nat_dec_eq(x_1, x_2);
return x_3;
}
}
static uint8_t _init_l_K22_MOG_ParityLiftRank_col__parity__sum__is__all__ones___nativeDecide__1() {
_start:
{
uint8_t x_1; 
x_1 = l_K22_MOG_ParityLiftRank_col__parity__sum__is__all__ones___nativeDecide__1___closed__2;
return x_1;
}
}
static uint8_t _init_l_K22_MOG_ParityLiftRank_parity__matrix__rank__eq__nine___nativeDecide__1___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; uint8_t x_3; 
x_1 = l_K22_MOG_ParityLiftRank_parityMatrixRank;
x_2 = lean_unsigned_to_nat(9u);
x_3 = lean_nat_dec_eq(x_1, x_2);
return x_3;
}
}
static uint8_t _init_l_K22_MOG_ParityLiftRank_parity__matrix__rank__eq__nine___nativeDecide__1() {
_start:
{
uint8_t x_1; 
x_1 = l_K22_MOG_ParityLiftRank_parity__matrix__rank__eq__nine___nativeDecide__1___closed__1;
return x_1;
}
}
static lean_object* _init_l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(0u);
x_2 = lean_mk_empty_array_with_capacity(x_1);
return x_2;
}
}
static lean_object* _init_l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__2() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; lean_object* x_4; 
x_1 = l_K22_MOG_ParityLiftRank_parityMasks;
x_2 = lean_unsigned_to_nat(9u);
x_3 = l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__1;
x_4 = l_List_takeTR_go___rarg(x_1, x_1, x_2, x_3);
return x_4;
}
}
static lean_object* _init_l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__3() {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__2;
x_2 = l_K22_MOG_ParityLiftRank_gf2Rank(x_1);
return x_2;
}
}
static uint8_t _init_l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__4() {
_start:
{
lean_object* x_1; lean_object* x_2; uint8_t x_3; 
x_1 = l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__3;
x_2 = lean_unsigned_to_nat(9u);
x_3 = lean_nat_dec_eq(x_1, x_2);
return x_3;
}
}
static uint8_t _init_l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1() {
_start:
{
uint8_t x_1; 
x_1 = l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__4;
return x_1;
}
}
static lean_object* _init_l_K22_MOG_ParityLiftRank_evenSumKernelDimension___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = lean_unsigned_to_nat(24u);
x_2 = lean_unsigned_to_nat(9u);
x_3 = lean_nat_sub(x_1, x_2);
return x_3;
}
}
static lean_object* _init_l_K22_MOG_ParityLiftRank_evenSumKernelDimension() {
_start:
{
lean_object* x_1; 
x_1 = l_K22_MOG_ParityLiftRank_evenSumKernelDimension___closed__1;
return x_1;
}
}
lean_object* initialize_Init(uint8_t builtin, lean_object*);
lean_object* initialize_Mathlib_Data_Nat_Bitwise(uint8_t builtin, lean_object*);
lean_object* initialize_Mathlib_Data_List_Range(uint8_t builtin, lean_object*);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_K22_MOG_ParityLiftRank(uint8_t builtin, lean_object* w) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Mathlib_Data_Nat_Bitwise(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Mathlib_Data_List_Range(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
l_K22_MOG_ParityLiftRank_rowParityMask___closed__1 = _init_l_K22_MOG_ParityLiftRank_rowParityMask___closed__1();
lean_mark_persistent(l_K22_MOG_ParityLiftRank_rowParityMask___closed__1);
l_K22_MOG_ParityLiftRank_colParityMask___closed__1 = _init_l_K22_MOG_ParityLiftRank_colParityMask___closed__1();
lean_mark_persistent(l_K22_MOG_ParityLiftRank_colParityMask___closed__1);
l_K22_MOG_ParityLiftRank_parityMasks = _init_l_K22_MOG_ParityLiftRank_parityMasks();
lean_mark_persistent(l_K22_MOG_ParityLiftRank_parityMasks);
l_K22_MOG_ParityLiftRank_gf2Rank___closed__1 = _init_l_K22_MOG_ParityLiftRank_gf2Rank___closed__1();
lean_mark_persistent(l_K22_MOG_ParityLiftRank_gf2Rank___closed__1);
l_K22_MOG_ParityLiftRank_parityMatrixRank___closed__1 = _init_l_K22_MOG_ParityLiftRank_parityMatrixRank___closed__1();
lean_mark_persistent(l_K22_MOG_ParityLiftRank_parityMatrixRank___closed__1);
l_K22_MOG_ParityLiftRank_parityMatrixRank = _init_l_K22_MOG_ParityLiftRank_parityMatrixRank();
lean_mark_persistent(l_K22_MOG_ParityLiftRank_parityMatrixRank);
l_K22_MOG_ParityLiftRank_row__col__parity__global__sum___nativeDecide__1 = _init_l_K22_MOG_ParityLiftRank_row__col__parity__global__sum___nativeDecide__1();
l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__1 = _init_l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__1();
lean_mark_persistent(l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__1);
l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__2 = _init_l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__2();
lean_mark_persistent(l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__2);
l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__3 = _init_l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__3();
lean_mark_persistent(l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__3);
l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__4 = _init_l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1___closed__4();
l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1 = _init_l_K22_MOG_ParityLiftRank_row__parity__sum__is__all__ones___nativeDecide__1();
l_K22_MOG_ParityLiftRank_col__parity__sum__is__all__ones___nativeDecide__1___closed__1 = _init_l_K22_MOG_ParityLiftRank_col__parity__sum__is__all__ones___nativeDecide__1___closed__1();
lean_mark_persistent(l_K22_MOG_ParityLiftRank_col__parity__sum__is__all__ones___nativeDecide__1___closed__1);
l_K22_MOG_ParityLiftRank_col__parity__sum__is__all__ones___nativeDecide__1___closed__2 = _init_l_K22_MOG_ParityLiftRank_col__parity__sum__is__all__ones___nativeDecide__1___closed__2();
l_K22_MOG_ParityLiftRank_col__parity__sum__is__all__ones___nativeDecide__1 = _init_l_K22_MOG_ParityLiftRank_col__parity__sum__is__all__ones___nativeDecide__1();
l_K22_MOG_ParityLiftRank_parity__matrix__rank__eq__nine___nativeDecide__1___closed__1 = _init_l_K22_MOG_ParityLiftRank_parity__matrix__rank__eq__nine___nativeDecide__1___closed__1();
l_K22_MOG_ParityLiftRank_parity__matrix__rank__eq__nine___nativeDecide__1 = _init_l_K22_MOG_ParityLiftRank_parity__matrix__rank__eq__nine___nativeDecide__1();
l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__1 = _init_l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__1();
lean_mark_persistent(l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__1);
l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__2 = _init_l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__2();
lean_mark_persistent(l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__2);
l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__3 = _init_l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__3();
lean_mark_persistent(l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__3);
l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__4 = _init_l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1___closed__4();
l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1 = _init_l_K22_MOG_ParityLiftRank_first__nine__masks__full__rank___nativeDecide__1();
l_K22_MOG_ParityLiftRank_evenSumKernelDimension___closed__1 = _init_l_K22_MOG_ParityLiftRank_evenSumKernelDimension___closed__1();
lean_mark_persistent(l_K22_MOG_ParityLiftRank_evenSumKernelDimension___closed__1);
l_K22_MOG_ParityLiftRank_evenSumKernelDimension = _init_l_K22_MOG_ParityLiftRank_evenSumKernelDimension();
lean_mark_persistent(l_K22_MOG_ParityLiftRank_evenSumKernelDimension);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
