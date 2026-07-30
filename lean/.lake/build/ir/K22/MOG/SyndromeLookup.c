// Lean compiler output
// Module: K22.MOG.SyndromeLookup
// Imports: Init Mathlib.Data.Finset.Basic Mathlib.Data.Finset.Card Mathlib.Data.Fintype.Basic
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
static lean_object* l_List_diff___at_K22_MOG_SyndromeLookup_symDiffCard___spec__2___closed__1;
lean_object* lean_mk_empty_array_with_capacity(lean_object*);
LEAN_EXPORT lean_object* l_K22_MOG_SyndromeLookup_mogAsTomczakLift___boxed(lean_object*);
LEAN_EXPORT lean_object* l_List_insert___at_K22_MOG_SyndromeLookup_symDiffCard___spec__7(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Array_foldrMUnsafe_fold___at_K22_MOG_SyndromeLookup_symDiffCard___spec__9___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lean_array_push(lean_object*, lean_object*);
uint8_t lean_usize_dec_eq(size_t, size_t);
LEAN_EXPORT lean_object* l_Multiset_ndunion___at_K22_MOG_SyndromeLookup_symDiffCard___spec__6(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_List_foldrTR___at_K22_MOG_SyndromeLookup_symDiffCard___spec__8(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_MOG_SyndromeLookup_mogSyndromeLookup___boxed(lean_object*);
size_t lean_usize_of_nat(lean_object*);
LEAN_EXPORT lean_object* l_K22_MOG_SyndromeLookup_symDiffCard(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Multiset_sub___at_K22_MOG_SyndromeLookup_symDiffCard___spec__1(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Array_foldrMUnsafe_fold___at_K22_MOG_SyndromeLookup_symDiffCard___spec__5(lean_object*, size_t, size_t, lean_object*);
LEAN_EXPORT uint8_t l_List_elem___at_K22_MOG_SyndromeLookup_symDiffCard___spec__3(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_List_eraseTR_go___at_K22_MOG_SyndromeLookup_symDiffCard___spec__4(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* l_List_lengthTRAux___rarg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Array_foldrMUnsafe_fold___at_K22_MOG_SyndromeLookup_symDiffCard___spec__5___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
uint8_t lean_nat_dec_eq(lean_object*, lean_object*);
uint8_t lean_nat_dec_lt(lean_object*, lean_object*);
size_t lean_usize_sub(size_t, size_t);
LEAN_EXPORT lean_object* l_List_diff___at_K22_MOG_SyndromeLookup_symDiffCard___spec__2(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_K22_MOG_SyndromeLookup_mogAsTomczakLift(lean_object*);
lean_object* lean_array_uget(lean_object*, size_t);
lean_object* l_List_redLength___rarg(lean_object*);
lean_object* lean_array_get_size(lean_object*);
LEAN_EXPORT lean_object* l_List_elem___at_K22_MOG_SyndromeLookup_symDiffCard___spec__3___boxed(lean_object*, lean_object*);
lean_object* l_List_toArrayAux___rarg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Array_foldrMUnsafe_fold___at_K22_MOG_SyndromeLookup_symDiffCard___spec__9(lean_object*, size_t, size_t, lean_object*);
LEAN_EXPORT lean_object* l_K22_MOG_SyndromeLookup_mogSyndromeLookup(lean_object*);
LEAN_EXPORT lean_object* l_K22_MOG_SyndromeLookup_mogSyndromeLookup(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_box(0);
return x_2;
}
}
LEAN_EXPORT lean_object* l_K22_MOG_SyndromeLookup_mogSyndromeLookup___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = l_K22_MOG_SyndromeLookup_mogSyndromeLookup(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT uint8_t l_List_elem___at_K22_MOG_SyndromeLookup_symDiffCard___spec__3(lean_object* x_1, lean_object* x_2) {
_start:
{
if (lean_obj_tag(x_2) == 0)
{
uint8_t x_3; 
x_3 = 0;
return x_3;
}
else
{
lean_object* x_4; lean_object* x_5; uint8_t x_6; 
x_4 = lean_ctor_get(x_2, 0);
x_5 = lean_ctor_get(x_2, 1);
x_6 = lean_nat_dec_eq(x_1, x_4);
if (x_6 == 0)
{
x_2 = x_5;
goto _start;
}
else
{
uint8_t x_8; 
x_8 = 1;
return x_8;
}
}
}
}
LEAN_EXPORT lean_object* l_Array_foldrMUnsafe_fold___at_K22_MOG_SyndromeLookup_symDiffCard___spec__5(lean_object* x_1, size_t x_2, size_t x_3, lean_object* x_4) {
_start:
{
uint8_t x_5; 
x_5 = lean_usize_dec_eq(x_2, x_3);
if (x_5 == 0)
{
size_t x_6; size_t x_7; lean_object* x_8; lean_object* x_9; 
x_6 = 1;
x_7 = lean_usize_sub(x_2, x_6);
x_8 = lean_array_uget(x_1, x_7);
x_9 = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(x_9, 0, x_8);
lean_ctor_set(x_9, 1, x_4);
x_2 = x_7;
x_4 = x_9;
goto _start;
}
else
{
return x_4;
}
}
}
LEAN_EXPORT lean_object* l_List_eraseTR_go___at_K22_MOG_SyndromeLookup_symDiffCard___spec__4(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
if (lean_obj_tag(x_3) == 0)
{
lean_dec(x_4);
lean_dec(x_2);
return x_1;
}
else
{
lean_object* x_5; lean_object* x_6; uint8_t x_7; 
x_5 = lean_ctor_get(x_3, 0);
lean_inc(x_5);
x_6 = lean_ctor_get(x_3, 1);
lean_inc(x_6);
lean_dec(x_3);
x_7 = lean_nat_dec_eq(x_5, x_2);
if (x_7 == 0)
{
lean_object* x_8; 
x_8 = lean_array_push(x_4, x_5);
x_3 = x_6;
x_4 = x_8;
goto _start;
}
else
{
lean_object* x_10; lean_object* x_11; uint8_t x_12; 
lean_dec(x_5);
lean_dec(x_2);
lean_dec(x_1);
x_10 = lean_array_get_size(x_4);
x_11 = lean_unsigned_to_nat(0u);
x_12 = lean_nat_dec_lt(x_11, x_10);
if (x_12 == 0)
{
lean_dec(x_10);
lean_dec(x_4);
return x_6;
}
else
{
size_t x_13; size_t x_14; lean_object* x_15; 
x_13 = lean_usize_of_nat(x_10);
lean_dec(x_10);
x_14 = 0;
x_15 = l_Array_foldrMUnsafe_fold___at_K22_MOG_SyndromeLookup_symDiffCard___spec__5(x_4, x_13, x_14, x_6);
lean_dec(x_4);
return x_15;
}
}
}
}
}
static lean_object* _init_l_List_diff___at_K22_MOG_SyndromeLookup_symDiffCard___spec__2___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(0u);
x_2 = lean_mk_empty_array_with_capacity(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_List_diff___at_K22_MOG_SyndromeLookup_symDiffCard___spec__2(lean_object* x_1, lean_object* x_2) {
_start:
{
if (lean_obj_tag(x_2) == 0)
{
return x_1;
}
else
{
lean_object* x_3; lean_object* x_4; uint8_t x_5; 
x_3 = lean_ctor_get(x_2, 0);
lean_inc(x_3);
x_4 = lean_ctor_get(x_2, 1);
lean_inc(x_4);
lean_dec(x_2);
x_5 = l_List_elem___at_K22_MOG_SyndromeLookup_symDiffCard___spec__3(x_3, x_1);
if (x_5 == 0)
{
lean_dec(x_3);
x_2 = x_4;
goto _start;
}
else
{
lean_object* x_7; lean_object* x_8; 
x_7 = l_List_diff___at_K22_MOG_SyndromeLookup_symDiffCard___spec__2___closed__1;
lean_inc(x_1);
x_8 = l_List_eraseTR_go___at_K22_MOG_SyndromeLookup_symDiffCard___spec__4(x_1, x_3, x_1, x_7);
x_1 = x_8;
x_2 = x_4;
goto _start;
}
}
}
}
LEAN_EXPORT lean_object* l_Multiset_sub___at_K22_MOG_SyndromeLookup_symDiffCard___spec__1(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = l_List_diff___at_K22_MOG_SyndromeLookup_symDiffCard___spec__2(x_1, x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* l_List_insert___at_K22_MOG_SyndromeLookup_symDiffCard___spec__7(lean_object* x_1, lean_object* x_2) {
_start:
{
uint8_t x_3; 
x_3 = l_List_elem___at_K22_MOG_SyndromeLookup_symDiffCard___spec__3(x_1, x_2);
if (x_3 == 0)
{
lean_object* x_4; 
x_4 = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(x_4, 0, x_1);
lean_ctor_set(x_4, 1, x_2);
return x_4;
}
else
{
lean_dec(x_1);
return x_2;
}
}
}
LEAN_EXPORT lean_object* l_Array_foldrMUnsafe_fold___at_K22_MOG_SyndromeLookup_symDiffCard___spec__9(lean_object* x_1, size_t x_2, size_t x_3, lean_object* x_4) {
_start:
{
uint8_t x_5; 
x_5 = lean_usize_dec_eq(x_2, x_3);
if (x_5 == 0)
{
size_t x_6; size_t x_7; lean_object* x_8; lean_object* x_9; 
x_6 = 1;
x_7 = lean_usize_sub(x_2, x_6);
x_8 = lean_array_uget(x_1, x_7);
x_9 = l_List_insert___at_K22_MOG_SyndromeLookup_symDiffCard___spec__7(x_8, x_4);
x_2 = x_7;
x_4 = x_9;
goto _start;
}
else
{
return x_4;
}
}
}
LEAN_EXPORT lean_object* l_List_foldrTR___at_K22_MOG_SyndromeLookup_symDiffCard___spec__8(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; uint8_t x_8; 
x_3 = l_List_redLength___rarg(x_2);
x_4 = lean_mk_empty_array_with_capacity(x_3);
lean_dec(x_3);
x_5 = l_List_toArrayAux___rarg(x_2, x_4);
x_6 = lean_array_get_size(x_5);
x_7 = lean_unsigned_to_nat(0u);
x_8 = lean_nat_dec_lt(x_7, x_6);
if (x_8 == 0)
{
lean_dec(x_6);
lean_dec(x_5);
return x_1;
}
else
{
size_t x_9; size_t x_10; lean_object* x_11; 
x_9 = lean_usize_of_nat(x_6);
lean_dec(x_6);
x_10 = 0;
x_11 = l_Array_foldrMUnsafe_fold___at_K22_MOG_SyndromeLookup_symDiffCard___spec__9(x_5, x_9, x_10, x_1);
lean_dec(x_5);
return x_11;
}
}
}
LEAN_EXPORT lean_object* l_Multiset_ndunion___at_K22_MOG_SyndromeLookup_symDiffCard___spec__6(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = l_List_foldrTR___at_K22_MOG_SyndromeLookup_symDiffCard___spec__8(x_2, x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* l_K22_MOG_SyndromeLookup_symDiffCard(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; 
lean_inc(x_2);
lean_inc(x_1);
x_3 = l_List_diff___at_K22_MOG_SyndromeLookup_symDiffCard___spec__2(x_1, x_2);
x_4 = l_List_diff___at_K22_MOG_SyndromeLookup_symDiffCard___spec__2(x_2, x_1);
x_5 = l_List_foldrTR___at_K22_MOG_SyndromeLookup_symDiffCard___spec__8(x_4, x_3);
x_6 = lean_unsigned_to_nat(0u);
x_7 = l_List_lengthTRAux___rarg(x_5, x_6);
lean_dec(x_5);
return x_7;
}
}
LEAN_EXPORT lean_object* l_List_elem___at_K22_MOG_SyndromeLookup_symDiffCard___spec__3___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
uint8_t x_3; lean_object* x_4; 
x_3 = l_List_elem___at_K22_MOG_SyndromeLookup_symDiffCard___spec__3(x_1, x_2);
lean_dec(x_2);
lean_dec(x_1);
x_4 = lean_box(x_3);
return x_4;
}
}
LEAN_EXPORT lean_object* l_Array_foldrMUnsafe_fold___at_K22_MOG_SyndromeLookup_symDiffCard___spec__5___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
size_t x_5; size_t x_6; lean_object* x_7; 
x_5 = lean_unbox_usize(x_2);
lean_dec(x_2);
x_6 = lean_unbox_usize(x_3);
lean_dec(x_3);
x_7 = l_Array_foldrMUnsafe_fold___at_K22_MOG_SyndromeLookup_symDiffCard___spec__5(x_1, x_5, x_6, x_4);
lean_dec(x_1);
return x_7;
}
}
LEAN_EXPORT lean_object* l_Array_foldrMUnsafe_fold___at_K22_MOG_SyndromeLookup_symDiffCard___spec__9___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
size_t x_5; size_t x_6; lean_object* x_7; 
x_5 = lean_unbox_usize(x_2);
lean_dec(x_2);
x_6 = lean_unbox_usize(x_3);
lean_dec(x_3);
x_7 = l_Array_foldrMUnsafe_fold___at_K22_MOG_SyndromeLookup_symDiffCard___spec__9(x_1, x_5, x_6, x_4);
lean_dec(x_1);
return x_7;
}
}
LEAN_EXPORT lean_object* l_K22_MOG_SyndromeLookup_mogAsTomczakLift(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_box(0);
return x_2;
}
}
LEAN_EXPORT lean_object* l_K22_MOG_SyndromeLookup_mogAsTomczakLift___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = l_K22_MOG_SyndromeLookup_mogAsTomczakLift(x_1);
lean_dec(x_1);
return x_2;
}
}
lean_object* initialize_Init(uint8_t builtin, lean_object*);
lean_object* initialize_Mathlib_Data_Finset_Basic(uint8_t builtin, lean_object*);
lean_object* initialize_Mathlib_Data_Finset_Card(uint8_t builtin, lean_object*);
lean_object* initialize_Mathlib_Data_Fintype_Basic(uint8_t builtin, lean_object*);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_K22_MOG_SyndromeLookup(uint8_t builtin, lean_object* w) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Mathlib_Data_Finset_Basic(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Mathlib_Data_Finset_Card(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Mathlib_Data_Fintype_Basic(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
l_List_diff___at_K22_MOG_SyndromeLookup_symDiffCard___spec__2___closed__1 = _init_l_List_diff___at_K22_MOG_SyndromeLookup_symDiffCard___spec__2___closed__1();
lean_mark_persistent(l_List_diff___at_K22_MOG_SyndromeLookup_symDiffCard___spec__2___closed__1);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
