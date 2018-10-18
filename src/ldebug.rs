use libc;
extern "C" {
    /*
    ** $Id: lstate.h,v 2.133.1.1 2017/04/19 17:39:34 roberto Exp $
    ** Global State
    ** See Copyright Notice in lua.h
    */
    /*

** Some notes about garbage-collected objects: All objects in Lua must
** be kept somehow accessible until being freed, so all objects always
** belong to one (and only one) of these lists, using field 'next' of
** the 'CommonHeader' for the link:
**
** 'allgc': all objects not marked for finalization;
** 'finobj': all objects marked for finalization;
** 'tobefnz': all objects ready to be finalized;
** 'fixedgc': all objects that are not to be collected (currently
** only small strings, such as reserved words).
**
** Moreover, there is another set of lists that control gray objects.
** These lists are linked by fields 'gclist'. (All objects that
** can become gray have such a field. The field is not the same
** in all objects, but it always has this name.)  Any gray object
** must belong to one of these lists, and all objects in these lists
** must be gray:
**
** 'gray': regular gray objects, still waiting to be visited.
** 'grayagain': objects that must be revisited at the atomic phase.
**   That includes
**   - black objects got in a write barrier;
**   - all kinds of weak tables during propagation phase;
**   - all threads.
** 'weak': tables with weak values to be cleared;
** 'ephemeron': ephemeron tables with white->white entries;
** 'allweak': tables with weak keys and/or weak values to be cleared.
** The last three lists are used only during the atomic phase.

*/
    /* defined in ldo.c */
    pub type lua_longjmp;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn luaH_new(L: *mut lua_State) -> *mut Table;
    #[no_mangle]
    fn luaH_setint(L: *mut lua_State, t: *mut Table, key: lua_Integer, value: *mut TValue) -> ();
    #[no_mangle]
    fn luaF_getlocalname(
        func: *const Proto,
        local_number: libc::c_int,
        pc: libc::c_int,
    ) -> *const libc::c_char;
    #[no_mangle]
    static luaP_opmodes: [lu_byte; 47];
    #[no_mangle]
    fn luaO_chunkid(out: *mut libc::c_char, source: *const libc::c_char, len: size_t) -> ();
    #[no_mangle]
    fn luaO_pushvfstring(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        argp: *mut __va_list_tag,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn luaO_pushfstring(L: *mut lua_State, fmt: *const libc::c_char, ...) -> *const libc::c_char;
    #[no_mangle]
    fn luaT_objtypename(L: *mut lua_State, o: *const TValue) -> *const libc::c_char;
    #[no_mangle]
    fn luaD_throw(L: *mut lua_State, errcode: libc::c_int) -> !;
    #[no_mangle]
    fn luaD_callnoyield(L: *mut lua_State, func: StkId, nResults: libc::c_int) -> ();
    #[no_mangle]
    fn luaC_step(L: *mut lua_State) -> ();
    #[no_mangle]
    fn luaV_tonumber_(obj: *const TValue, n: *mut lua_Number) -> libc::c_int;
    #[no_mangle]
    fn luaV_tointeger(obj: *const TValue, p: *mut lua_Integer, mode: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaD_hook(L: *mut lua_State, event: libc::c_int, line: libc::c_int) -> ();
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __sig_atomic_t = libc::c_int;
pub type intptr_t = libc::c_long;
/*
** $Id: lua.h,v 1.332.1.2 2018/06/13 16:58:17 roberto Exp $
** Lua - A Scripting Language
** Lua.org, PUC-Rio, Brazil (http://www.lua.org)
** See Copyright Notice at the end of this file
*/
/* mark for precompiled code ('<esc>Lua') */
/* option for multiple returns in 'lua_pcall' and 'lua_call' */
/*
** Pseudo-indices
** (-LUAI_MAXSTACK is the minimum valid index; we keep some free empty
** space after that to help overflow detection)
*/
/* thread status */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_State {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nci: libc::c_ushort,
    pub status: lu_byte,
    pub top: StkId,
    pub l_G: *mut global_State,
    pub ci: *mut CallInfo,
    pub oldpc: *const Instruction,
    pub stack_last: StkId,
    pub stack: StkId,
    pub openupval: *mut UpVal,
    pub gclist: *mut GCObject,
    pub twups: *mut lua_State,
    pub errorJmp: *mut lua_longjmp,
    pub base_ci: CallInfo,
    pub hook: lua_Hook,
    pub errfunc: ptrdiff_t,
    pub stacksize: libc::c_int,
    pub basehookcount: libc::c_int,
    pub hookcount: libc::c_int,
    pub nny: libc::c_ushort,
    pub nCcalls: libc::c_ushort,
    pub hookmask: sig_atomic_t,
    pub allowhook: lu_byte,
}
/* 16-bit ints */
 /* }{ */
/* } */
/* chars used as small naturals (so that 'char' is reserved for characters) */
pub type lu_byte = libc::c_uchar;
pub type sig_atomic_t = __sig_atomic_t;
/* Functions to be called by the debugger in specific events */
pub type lua_Hook = Option<unsafe extern "C" fn(_: *mut lua_State, _: *mut lua_Debug) -> ()>;
/*
** {==============================================================
** some useful macros
** ===============================================================
*/
/* }============================================================== */
/*
** {==============================================================
** compatibility macros for unsigned conversions
** ===============================================================
*/
/* }============================================================== */
/*
** {======================================================================
** Debug API
** =======================================================================
*/
/*
** Event codes
*/
/*
** Event masks
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_Debug {
    pub event: libc::c_int,
    pub name: *const libc::c_char,
    pub namewhat: *const libc::c_char,
    pub what: *const libc::c_char,
    pub source: *const libc::c_char,
    pub currentline: libc::c_int,
    pub linedefined: libc::c_int,
    pub lastlinedefined: libc::c_int,
    pub nups: libc::c_uchar,
    pub nparams: libc::c_uchar,
    pub isvararg: libc::c_char,
    pub istailcall: libc::c_char,
    pub short_src: [libc::c_char; 60],
    pub i_ci: *mut CallInfo,
}
/* private part */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfo {
    pub func: StkId,
    pub top: StkId,
    pub previous: *mut CallInfo,
    pub next: *mut CallInfo,
    pub u: unnamed,
    pub extra: ptrdiff_t,
    pub nresults: libc::c_short,
    pub callstatus: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed {
    pub l: unnamed_1,
    pub c: unnamed_0,
}
/* only for C functions */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_0 {
    pub k: lua_KFunction,
    pub old_errfunc: ptrdiff_t,
    pub ctx: lua_KContext,
}
/* type for continuation-function contexts */
pub type lua_KContext = intptr_t;
/*
** Type for continuation functions
*/
pub type lua_KFunction =
    Option<unsafe extern "C" fn(_: *mut lua_State, _: libc::c_int, _: lua_KContext) -> libc::c_int>;
/* only for Lua functions */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_1 {
    pub base: StkId,
    pub savedpc: *const Instruction,
}
/* internal assertions for in-house debugging */
/*
** assertion for checking API calls
*/
/* macro to avoid warnings about unused variables */
/* type casts (a macro highlights casts in the code) */
/* cast a signed lua_Integer to lua_Unsigned */
/*
** cast a lua_Unsigned to a signed lua_Integer; this cast is
** not strict ISO C, but two-complement architectures should
** work fine.
*/
/*
** non-return type
*/
/*
** maximum depth for nested C calls and syntactical nested non-terminals
** in a program. (Value must fit in an unsigned short int.)
*/
/*
** type for virtual-machine instructions;
** must be an unsigned with (at least) 4 bytes (see details in lopcodes.h)
*/
pub type Instruction = libc::c_uint;
/* macro defining a nil value */
/* raw type tag of a TValue */
/* tag with no variants (bits 0-3) */
/* type tag of a TValue (bits 0-3 for tags + variant bits 4-5) */
/* type tag of a TValue with no variants (bits 0-3) */
/* Macros to test type */
/* Macros to access values */
/* a dead value may get the 'gc' field, but cannot access its contents */
/* Macros for internal tests */
/* Macros to set values */
/*
** different types of assignments, according to destination
*/
/* from stack to (same) stack */
/* to stack (not from same stack) */
/* from table to same table */
/* to new object */
/* to table (define it as an expression to be used in macros) */
/*
** {======================================================
** types and prototypes
** =======================================================
*/
/* index to stack elements */
pub type StkId = *mut TValue;
pub type TValue = lua_TValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_TValue {
    pub value_: Value,
    pub tt_: libc::c_int,
}
/*
** Tagged Values. This is the basic representation of values in Lua,
** an actual value plus a tag with its type.
*/
/*
** Union of all Lua values
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub union Value {
    pub gc: *mut GCObject,
    pub p: *mut libc::c_void,
    pub b: libc::c_int,
    pub f: lua_CFunction,
    pub i: lua_Integer,
    pub n: lua_Number,
}
/*
** basic types
*/
/* minimum Lua stack available to a C function */
/* predefined values in the registry */
/* type of numbers in Lua */
pub type lua_Number = libc::c_double;
/* type for integer functions */
pub type lua_Integer = libc::c_longlong;
/*
** Type for C functions registered with Lua
*/
pub type lua_CFunction = Option<unsafe extern "C" fn(_: *mut lua_State) -> libc::c_int>;
/*
** $Id: lobject.h,v 2.117.1.1 2017/04/19 17:39:34 roberto Exp $
** Type definitions for Lua objects
** See Copyright Notice in lua.h
*/
/*
** Extra tags for non-values
*/
/* function prototypes */
/* removed keys in tables */
/*
** number of all possible tags (including LUA_TNONE but excluding DEADKEY)
*/
/*
** tags for Tagged Values have the following use of bits:
** bits 0-3: actual tag (a LUA_T* value)
** bits 4-5: variant bits
** bit 6: whether value is collectable
*/
/*
** LUA_TFUNCTION variants:
** 0 - Lua function
** 1 - light C function
** 2 - regular C function (closure)
*/
/* Variant tags for functions */
/* Lua closure */
/* light C function */
/* C closure */
/* Variant tags for strings */
/* short strings */
/* long strings */
/* Variant tags for numbers */
/* float numbers */
/* integer numbers */
/* Bit mark for collectable types */
/* mark a tag as collectable */
/*
** Common type for all collectable objects
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GCObject {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
}
/*
** Lua Upvalues
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpVal {
    pub v: *mut TValue,
    pub refcount: lu_mem,
    pub u: unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_2 {
    pub open: unnamed_3,
    pub value: TValue,
}
/* (when open) */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_3 {
    pub next: *mut UpVal,
    pub touched: libc::c_int,
}
/*
** $Id: llimits.h,v 1.141.1.1 2017/04/19 17:20:42 roberto Exp $
** Limits, basic types, and some other 'installation-dependent' definitions
** See Copyright Notice in lua.h
*/
/*
** 'lu_mem' and 'l_mem' are unsigned/signed integers big enough to count
** the total memory used by Lua (in bytes). Usually, 'size_t' and
** 'ptrdiff_t' should work, but we use 'long' for 16-bit machines.
*/
/* { external definitions? */
/* }{ */
pub type lu_mem = size_t;
/*
** Bits in CallInfo status
*/
/* original value of 'allowhook' */
/* call is running a Lua function */
/* call is running a debug hook */
/* call is running on a fresh invocation
                                   of luaV_execute */
/* call is a yieldable protected call */
/* call was tail called */
/* last hook called yielded */
/* using __lt for __le */
/* call is running a finalizer */
/* assume that CIST_OAH has offset 0 and that 'v' is strictly 0/1 */
/*
** 'global state', shared by all threads of this state
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct global_State {
    pub frealloc: lua_Alloc,
    pub ud: *mut libc::c_void,
    pub totalbytes: l_mem,
    pub GCdebt: l_mem,
    pub GCmemtrav: lu_mem,
    pub GCestimate: lu_mem,
    pub strt: stringtable,
    pub l_registry: TValue,
    pub seed: libc::c_uint,
    pub currentwhite: lu_byte,
    pub gcstate: lu_byte,
    pub gckind: lu_byte,
    pub gcrunning: lu_byte,
    pub allgc: *mut GCObject,
    pub sweepgc: *mut *mut GCObject,
    pub finobj: *mut GCObject,
    pub gray: *mut GCObject,
    pub grayagain: *mut GCObject,
    pub weak: *mut GCObject,
    pub ephemeron: *mut GCObject,
    pub allweak: *mut GCObject,
    pub tobefnz: *mut GCObject,
    pub fixedgc: *mut GCObject,
    pub twups: *mut lua_State,
    pub gcfinnum: libc::c_uint,
    pub gcpause: libc::c_int,
    pub gcstepmul: libc::c_int,
    pub panic: lua_CFunction,
    pub mainthread: *mut lua_State,
    pub version: *const lua_Number,
    pub memerrmsg: *mut TString,
    pub tmname: [*mut TString; 24],
    pub mt: [*mut Table; 9],
    pub strcache: [[*mut TString; 2]; 53],
}
/*
** Header for string value; string bytes follow the end of this structure
** (aligned according to 'UTString'; see next).
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TString {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub extra: lu_byte,
    pub shrlen: lu_byte,
    pub hash: libc::c_uint,
    pub u: unnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_4 {
    pub lnglen: size_t,
    pub hnext: *mut TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Table {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub flags: lu_byte,
    pub lsizenode: lu_byte,
    pub sizearray: libc::c_uint,
    pub array: *mut TValue,
    pub node: *mut Node,
    pub lastfree: *mut Node,
    pub metatable: *mut Table,
    pub gclist: *mut GCObject,
}
/* copy a value into a key without messing up field 'next' */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub i_val: TValue,
    pub i_key: TKey,
}
/*
** Tables
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub union TKey {
    pub nk: unnamed_5,
    pub tvk: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_5 {
    pub value_: Value,
    pub tt_: libc::c_int,
    pub next: libc::c_int,
}
/*
** Atomic type (relative to signals) to better ensure that 'lua_sethook'
** is thread safe
*/
/* extra stack space to handle TM calls and some other extras */
/* kinds of Garbage Collection */
/* gc was forced by an allocation failure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stringtable {
    pub hash: *mut *mut TString,
    pub nuse: libc::c_int,
    pub size: libc::c_int,
}
pub type l_mem = ptrdiff_t;
/*
** Type for memory-allocation functions
*/
pub type lua_Alloc = Option<
    unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void, _: size_t, _: size_t)
        -> *mut libc::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Closure {
    pub c: CClosure,
    pub l: LClosure,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LClosure {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub p: *mut Proto,
    pub upvals: [*mut UpVal; 1],
}
/*
** Function Prototypes
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Proto {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub numparams: lu_byte,
    pub is_vararg: lu_byte,
    pub maxstacksize: lu_byte,
    pub sizeupvalues: libc::c_int,
    pub sizek: libc::c_int,
    pub sizecode: libc::c_int,
    pub sizelineinfo: libc::c_int,
    pub sizep: libc::c_int,
    pub sizelocvars: libc::c_int,
    pub linedefined: libc::c_int,
    pub lastlinedefined: libc::c_int,
    pub k: *mut TValue,
    pub code: *mut Instruction,
    pub p: *mut *mut Proto,
    pub lineinfo: *mut libc::c_int,
    pub locvars: *mut LocVar,
    pub upvalues: *mut Upvaldesc,
    pub cache: *mut LClosure,
    pub source: *mut TString,
    pub gclist: *mut GCObject,
}
/*
**  Get the address of memory block inside 'Udata'.
** (Access to 'ttuv_' ensures that value is really a 'Udata'.)
*/
/*
** Description of an upvalue for function prototypes
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Upvaldesc {
    pub name: *mut TString,
    pub instack: lu_byte,
    pub idx: lu_byte,
}
/*
** Description of a local variable for function prototypes
** (used for debug information)
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LocVar {
    pub varname: *mut TString,
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
}
/*
** Closures
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CClosure {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub f: lua_CFunction,
    pub upvalue: [TValue; 1],
}
/*
** Union of all collectable objects (only for conversions)
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub union GCUnion {
    pub gc: GCObject,
    pub ts: TString,
    pub u: Udata,
    pub cl: Closure,
    pub h: Table,
    pub p: Proto,
    pub th: lua_State,
}
/*
** Get the actual string (array of bytes) from a 'TString'.
** (Access to 'extra' ensures that value is really a 'TString'.)
*/
/* get the actual string (array of bytes) from a Lua value */
/* get string length from 'TString *s' */
/* get string length from 'TValue *o' */
/*
** Header for userdata; memory area follows the end of this structure
** (aligned according to 'UUdata'; see next).
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Udata {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub ttuv_: lu_byte,
    pub metatable: *mut Table,
    pub len: size_t,
    pub user_: Value,
}
/*
** Ensures that address after this type is always fully aligned.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub union UTString {
    pub dummy: L_Umaxalign,
    pub tsv: TString,
}
/* maximum value for size_t */
/* maximum size visible for Lua (must be representable in a lua_Integer */
/* maximum value of an int */
/*
** conversion of pointer to unsigned integer:
** this is for hashing only; there is no problem if the integer
** cannot hold the whole pointer value
*/
/* type to ensure maximum alignment */
#[derive(Copy, Clone)]
#[repr(C)]
pub union L_Umaxalign {
    pub n: lua_Number,
    pub u: libc::c_double,
    pub s: *mut libc::c_void,
    pub i: lua_Integer,
    pub l: libc::c_long,
}
/*
** $Id: ltm.h,v 2.22.1.1 2017/04/19 17:20:42 roberto Exp $
** Tag methods
** See Copyright Notice in lua.h
*/
/*
* WARNING: if you change the order of this enumeration,
* grep "ORDER TM" and "ORDER OP"
*/
pub type TMS = libc::c_uint;
/* number of elements in the enum */
pub const TM_N: TMS = 24;
pub const TM_CALL: TMS = 23;
pub const TM_CONCAT: TMS = 22;
pub const TM_LE: TMS = 21;
pub const TM_LT: TMS = 20;
pub const TM_BNOT: TMS = 19;
pub const TM_UNM: TMS = 18;
pub const TM_SHR: TMS = 17;
pub const TM_SHL: TMS = 16;
pub const TM_BXOR: TMS = 15;
pub const TM_BOR: TMS = 14;
pub const TM_BAND: TMS = 13;
pub const TM_IDIV: TMS = 12;
pub const TM_DIV: TMS = 11;
pub const TM_POW: TMS = 10;
pub const TM_MOD: TMS = 9;
pub const TM_MUL: TMS = 8;
pub const TM_SUB: TMS = 7;
pub const TM_ADD: TMS = 6;
/* last tag method with fast access */
pub const TM_EQ: TMS = 5;
pub const TM_LEN: TMS = 4;
pub const TM_MODE: TMS = 3;
pub const TM_GC: TMS = 2;
pub const TM_NEWINDEX: TMS = 1;
pub const TM_INDEX: TMS = 0;
/*	A B C	if ((RK(B) <= RK(C)) ~= A) then pc++		*/
pub const OP_LE: OpCode = 33;
/*	A B C	if ((RK(B) <  RK(C)) ~= A) then pc++		*/
pub const OP_LT: OpCode = 32;
/*	A B C	if ((RK(B) == RK(C)) ~= A) then pc++		*/
pub const OP_EQ: OpCode = 31;
/*	A B C	R(A) := R(B).. ... ..R(C)			*/
pub const OP_CONCAT: OpCode = 29;
/*	A B	R(A) := length of R(B)				*/
pub const OP_LEN: OpCode = 28;
/*	A B	R(A) := ~R(B)					*/
pub const OP_BNOT: OpCode = 26;
/*	A B	R(A) := -R(B)					*/
pub const OP_UNM: OpCode = 25;
/*	A B C	R(A) := RK(B) + RK(C)				*/
pub const OP_ADD: OpCode = 13;
/*
** size and position of opcode arguments.
*/
/*
** limits for opcode arguments.
** we use (signed) int to manipulate most arguments,
** so they must fit in LUAI_BITSINT-1 bits (-1 for sign)
*/
/* 'sBx' is signed */
/* creates a mask with 'n' 1 bits at position 'p' */
/* creates a mask with 'n' 0 bits at position 'p' */
/*
** the following macros help to manipulate instructions
*/
/*
** Macros to operate RK indices
*/
/* this bit 1 means constant (0 means register) */
/* test whether value is a constant */
/* gets the index of the constant */
/* (for debugging only) */
/* code a constant index as a RK value */
/*
** invalid register that fits in 8 bits
*/
/*
** R(x) - register
** Kst(x) - constant (in constant table)
** RK(x) == if ISK(x) then Kst(INDEXK(x)) else R(x)
*/
/*
** grep "ORDER OP" if you change these enums
*/
pub type OpCode = libc::c_uint;
/*	Ax	extra (larger) argument for previous opcode	*/
pub const OP_EXTRAARG: OpCode = 46;
/*	A B	R(A), R(A+1), ..., R(A+B-2) = vararg		*/
pub const OP_VARARG: OpCode = 45;
/*	A Bx	R(A) := closure(KPROTO[Bx])			*/
pub const OP_CLOSURE: OpCode = 44;
/*	A B C	R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B	*/
pub const OP_SETLIST: OpCode = 43;
/*	A sBx	if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx }*/
pub const OP_TFORLOOP: OpCode = 42;
/*	A C	R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));	*/
pub const OP_TFORCALL: OpCode = 41;
/*	A sBx	R(A)-=R(A+2); pc+=sBx				*/
pub const OP_FORPREP: OpCode = 40;
/*	A sBx	R(A)+=R(A+2);
			if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }*/
pub const OP_FORLOOP: OpCode = 39;
/*	A B	return R(A), ... ,R(A+B-2)	(see note)	*/
pub const OP_RETURN: OpCode = 38;
/*	A B C	return R(A)(R(A+1), ... ,R(A+B-1))		*/
pub const OP_TAILCALL: OpCode = 37;
/*	A B C	R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1)) */
pub const OP_CALL: OpCode = 36;
/*	A B C	if (R(B) <=> C) then R(A) := R(B) else pc++	*/
pub const OP_TESTSET: OpCode = 35;
/*	A C	if not (R(A) <=> C) then pc++			*/
pub const OP_TEST: OpCode = 34;
/*	A sBx	pc+=sBx; if (A) close all upvalues >= R(A - 1)	*/
pub const OP_JMP: OpCode = 30;
/*	A B	R(A) := not R(B)				*/
pub const OP_NOT: OpCode = 27;
/*	A B C	R(A) := RK(B) >> RK(C)				*/
pub const OP_SHR: OpCode = 24;
/*	A B C	R(A) := RK(B) << RK(C)				*/
pub const OP_SHL: OpCode = 23;
/*	A B C	R(A) := RK(B) ~ RK(C)				*/
pub const OP_BXOR: OpCode = 22;
/*	A B C	R(A) := RK(B) | RK(C)				*/
pub const OP_BOR: OpCode = 21;
/*	A B C	R(A) := RK(B) & RK(C)				*/
pub const OP_BAND: OpCode = 20;
/*	A B C	R(A) := RK(B) // RK(C)				*/
pub const OP_IDIV: OpCode = 19;
/*	A B C	R(A) := RK(B) / RK(C)				*/
pub const OP_DIV: OpCode = 18;
/*	A B C	R(A) := RK(B) ^ RK(C)				*/
pub const OP_POW: OpCode = 17;
/*	A B C	R(A) := RK(B) % RK(C)				*/
pub const OP_MOD: OpCode = 16;
/*	A B C	R(A) := RK(B) * RK(C)				*/
pub const OP_MUL: OpCode = 15;
/*	A B C	R(A) := RK(B) - RK(C)				*/
pub const OP_SUB: OpCode = 14;
/*	A B C	R(A+1) := R(B); R(A) := R(B)[RK(C)]		*/
pub const OP_SELF: OpCode = 12;
/*	A B C	R(A) := {} (size = B,C)				*/
pub const OP_NEWTABLE: OpCode = 11;
/*	A B C	R(A)[RK(B)] := RK(C)				*/
pub const OP_SETTABLE: OpCode = 10;
/*	A B	UpValue[B] := R(A)				*/
pub const OP_SETUPVAL: OpCode = 9;
/*	A B C	UpValue[A][RK(B)] := RK(C)			*/
pub const OP_SETTABUP: OpCode = 8;
/*	A B C	R(A) := R(B)[RK(C)]				*/
pub const OP_GETTABLE: OpCode = 7;
/*	A B C	R(A) := UpValue[B][RK(C)]			*/
pub const OP_GETTABUP: OpCode = 6;
/*	A B	R(A) := UpValue[B]				*/
pub const OP_GETUPVAL: OpCode = 5;
/*	A B	R(A), R(A+1), ..., R(A+B) := nil		*/
pub const OP_LOADNIL: OpCode = 4;
/*	A B C	R(A) := (Bool)B; if (C) pc++			*/
pub const OP_LOADBOOL: OpCode = 3;
/*	A 	R(A) := Kst(extra arg)				*/
pub const OP_LOADKX: OpCode = 2;
/*	A Bx	R(A) := Kst(Bx)					*/
pub const OP_LOADK: OpCode = 1;
/*----------------------------------------------------------------------
name		args	description
------------------------------------------------------------------------*/
/*	A B	R(A) := R(B)					*/
pub const OP_MOVE: OpCode = 0;
#[no_mangle]
pub unsafe extern "C" fn lua_getstack(
    mut L: *mut lua_State,
    mut level: libc::c_int,
    mut ar: *mut lua_Debug,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    if level < 0i32 {
        /* invalid (negative) level */
        return 0i32;
    } else {
        ci = (*L).ci;
        while level > 0i32 && ci != &mut (*L).base_ci as *mut CallInfo {
            level -= 1;
            ci = (*ci).previous
        }
        if level == 0i32 && ci != &mut (*L).base_ci as *mut CallInfo {
            /* level found? */
            status = 1i32;
            (*ar).i_ci = ci
        } else {
            status = 0i32
        }
        return status;
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_getinfo(
    mut L: *mut lua_State,
    mut what: *const libc::c_char,
    mut ar: *mut lua_Debug,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut cl: *mut Closure = 0 as *mut Closure;
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    let mut func: StkId = 0 as *mut TValue;
    swapextra(L);
    if *what as libc::c_int == '>' as i32 {
        ci = 0 as *mut CallInfo;
        func = (*L).top.offset(-1isize);
        /* skip the '>' */
        what = what.offset(1isize);
        /* pop function */
        (*L).top = (*L).top.offset(-1isize)
    } else {
        ci = (*ar).i_ci;
        func = (*ci).func
    }
    cl = if (*func).tt_ & 0x1fi32 == 6i32 {
        &mut (*((*func).value_.gc as *mut GCUnion)).cl
    } else {
        0 as *mut Closure
    };
    status = auxgetinfo(L, what, ar, cl, ci);
    if !strchr(what, 'f' as i32).is_null() {
        let mut io1: *mut TValue = (*L).top;
        *io1 = *func;
        (*L).top = (*L).top.offset(1isize)
    }
    /* correct before option 'L', which can raise a mem. error */
    swapextra(L);
    if !strchr(what, 'L' as i32).is_null() {
        collectvalidlines(L, cl);
    }
    return status;
}
unsafe extern "C" fn collectvalidlines(mut L: *mut lua_State, mut f: *mut Closure) -> () {
    if f.is_null() || (*f).c.tt as libc::c_int == 6i32 | 2i32 << 4i32 {
        (*(*L).top).tt_ = 0i32;
        (*L).top = (*L).top.offset(1isize)
    } else {
        let mut i: libc::c_int = 0;
        let mut v: TValue = lua_TValue {
            value_: Value {
                gc: 0 as *mut GCObject,
            },
            tt_: 0,
        };
        let mut lineinfo: *mut libc::c_int = (*(*f).l.p).lineinfo;
        /* new table to store active lines */
        let mut t: *mut Table = luaH_new(L);
        let mut io: *mut TValue = (*L).top;
        /* push it on stack */
        let mut x_: *mut Table = t;
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = 5i32 | 1i32 << 6i32;
        (*L).top = (*L).top.offset(1isize);
        /* boolean 'true' to be the value of all indices */
        let mut io_0: *mut TValue = &mut v;
        (*io_0).value_.b = 1i32;
        (*io_0).tt_ = 1i32;
        /* for all lines with code */
        i = 0i32;
        while i < (*(*f).l.p).sizelineinfo {
            /* table[line] = true */
            luaH_setint(L, t, *lineinfo.offset(i as isize) as lua_Integer, &mut v);
            i += 1
        }
    };
}
/*
** If function yielded, its 'func' can be in the 'extra' field. The
** next function restores 'func' to its correct value for debugging
** purposes. (It exchanges 'func' and 'extra'; so, when called again,
** after debugging, it also "re-restores" ** 'func' to its altered value.
*/
unsafe extern "C" fn swapextra(mut L: *mut lua_State) -> () {
    if (*L).status as libc::c_int == 1i32 {
        /* get function that yielded */
        let mut ci: *mut CallInfo = (*L).ci;
        /* exchange its 'func' and 'extra' values */
        let mut temp: StkId = (*ci).func;
        (*ci).func = ((*L).stack as *mut libc::c_char).offset((*ci).extra as isize) as *mut TValue;
        (*ci).extra = (temp as *mut libc::c_char)
            .wrapping_offset_from((*L).stack as *mut libc::c_char)
            as libc::c_long
    };
}
unsafe extern "C" fn auxgetinfo(
    mut L: *mut lua_State,
    mut what: *const libc::c_char,
    mut ar: *mut lua_Debug,
    mut f: *mut Closure,
    mut ci: *mut CallInfo,
) -> libc::c_int {
    let mut status: libc::c_int = 1i32;
    while 0 != *what {
        match *what as libc::c_int {
            83 => {
                funcinfo(ar, f);
            }
            108 => {
                (*ar).currentline =
                    if !ci.is_null() && 0 != (*ci).callstatus as libc::c_int & 1i32 << 1i32 {
                        currentline(ci)
                    } else {
                        -1i32
                    }
            }
            117 => {
                (*ar).nups = (if f.is_null() {
                    0i32
                } else {
                    (*f).c.nupvalues as libc::c_int
                }) as libc::c_uchar;
                if f.is_null() || (*f).c.tt as libc::c_int == 6i32 | 2i32 << 4i32 {
                    (*ar).isvararg = 1i32 as libc::c_char;
                    (*ar).nparams = 0i32 as libc::c_uchar
                } else {
                    (*ar).isvararg = (*(*f).l.p).is_vararg as libc::c_char;
                    (*ar).nparams = (*(*f).l.p).numparams
                }
            }
            116 => {
                (*ar).istailcall = (if !ci.is_null() {
                    (*ci).callstatus as libc::c_int & 1i32 << 5i32
                } else {
                    0i32
                }) as libc::c_char
            }
            110 => {
                (*ar).namewhat = getfuncname(L, ci, &mut (*ar).name);
                if (*ar).namewhat.is_null() {
                    /* not found */
                    (*ar).namewhat = b"\x00" as *const u8 as *const libc::c_char;
                    (*ar).name = 0 as *const libc::c_char
                }
            }
            76 => {}
            102 => {}
            _ => {
                /* handled by lua_getinfo */
                /* invalid option */
                status = 0i32
            }
        }
        what = what.offset(1isize)
    }
    return status;
}
unsafe extern "C" fn getfuncname(
    mut L: *mut lua_State,
    mut ci: *mut CallInfo,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    /* no 'ci'? */
    if ci.is_null() {
        /* no info */
        return 0 as *const libc::c_char;
    } else if 0 != (*ci).callstatus as libc::c_int & 1i32 << 8i32 {
        /* is this a finalizer? */
        *name = b"__gc\x00" as *const u8 as *const libc::c_char;
        /* report it as such */
        return b"metamethod\x00" as *const u8 as *const libc::c_char;
    } else if 0 == (*ci).callstatus as libc::c_int & 1i32 << 5i32
        && 0 != (*(*ci).previous).callstatus as libc::c_int & 1i32 << 1i32
    {
        return funcnamefromcode(L, (*ci).previous, name);
    } else {
        return 0 as *const libc::c_char;
    };
}
/*
** $Id: ldebug.c,v 2.121.1.2 2017/07/10 17:21:50 roberto Exp $
** Debug Interface
** See Copyright Notice in lua.h
*/
/* Active Lua function (given call info) */
unsafe extern "C" fn funcnamefromcode(
    mut L: *mut lua_State,
    mut ci: *mut CallInfo,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    /* (initial value avoids warnings) */
    let mut tm: TMS = TM_INDEX;
    /* calling function */
    let mut p: *mut Proto = (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l.p;
    /* calling instruction index */
    let mut pc: libc::c_int = currentpc(ci);
    /* calling instruction */
    let mut i: Instruction = *(*p).code.offset(pc as isize);
    if 0 != (*ci).callstatus as libc::c_int & 1i32 << 2i32 {
        /* was it called inside a hook? */
        *name = b"?\x00" as *const u8 as *const libc::c_char;
        return b"hook\x00" as *const u8 as *const libc::c_char;
    } else {
        match (i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode as libc::c_uint {
            36 | 37 => {
                /* get function name */
                return getobjname(
                    p,
                    pc,
                    (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32) as libc::c_int,
                    name,
                );
            }
            41 => {
                /* for iterator */
                *name = b"for iterator\x00" as *const u8 as *const libc::c_char;
                return b"for iterator\x00" as *const u8 as *const libc::c_char;
            }
            12 | 6 | 7 => tm = TM_INDEX,
            8 | 10 => tm = TM_NEWINDEX,
            13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 => {
                /* ORDER OP */
                let mut offset: libc::c_int =
                    (i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode
                        as libc::c_int - OP_ADD as libc::c_int;
                /* ORDER TM */
                tm = (offset + TM_ADD as libc::c_int) as TMS
            }
            25 => tm = TM_UNM,
            26 => tm = TM_BNOT,
            28 => tm = TM_LEN,
            29 => tm = TM_CONCAT,
            31 => tm = TM_EQ,
            32 => tm = TM_LT,
            33 => tm = TM_LE,
            _ => {
                /* cannot find a reasonable name */
                return 0 as *const libc::c_char;
            }
        }
        *name = ((*(*L).l_G).tmname[tm as usize] as *mut libc::c_char)
            .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
        return b"metamethod\x00" as *const u8 as *const libc::c_char;
    };
}
unsafe extern "C" fn currentpc(mut ci: *mut CallInfo) -> libc::c_int {
    return (*ci)
        .u
        .l
        .savedpc
        .wrapping_offset_from((*(*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l.p).code)
        as libc::c_long as libc::c_int - 1i32;
}
/*
** {======================================================
** Symbolic Execution
** =======================================================
*/
unsafe extern "C" fn getobjname(
    mut p: *mut Proto,
    mut lastpc: libc::c_int,
    mut reg: libc::c_int,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut pc: libc::c_int = 0;
    *name = luaF_getlocalname(p, reg + 1i32, lastpc);
    /* is a local? */
    if !(*name).is_null() {
        return b"local\x00" as *const u8 as *const libc::c_char;
    } else {
        /* else try symbolic execution */
        pc = findsetreg(p, lastpc, reg);
        if pc != -1i32 {
            /* could find instruction? */
            let mut i: Instruction = *(*p).code.offset(pc as isize);
            let mut op: OpCode =
                (i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode;
            match op as libc::c_uint {
                0 => {
                    /* move from 'b' to 'a' */
                    let mut b: libc::c_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int;
                    if b < (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                        as libc::c_int
                    {
                        /* get name for 'b' */
                        return getobjname(p, pc, b, name);
                    }
                }
                6 | 7 => {
                    /* key index */
                    let mut k: libc::c_int = (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int;
                    /* table index */
                    let mut t: libc::c_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int;
                    /* name of indexed variable */
                    let mut vn: *const libc::c_char =
                        if op as libc::c_uint == OP_GETTABLE as libc::c_int as libc::c_uint {
                            luaF_getlocalname(p, t + 1i32, pc)
                        } else {
                            upvalname(p, t)
                        };
                    kname(p, pc, k, name);
                    return if !vn.is_null()
                        && strcmp(vn, b"_ENV\x00" as *const u8 as *const libc::c_char) == 0i32
                    {
                        b"global\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"field\x00" as *const u8 as *const libc::c_char
                    };
                }
                5 => {
                    *name = upvalname(
                        p,
                        (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as libc::c_int,
                    );
                    return b"upvalue\x00" as *const u8 as *const libc::c_char;
                }
                1 | 2 => {
                    let mut b_0: libc::c_int =
                        if op as libc::c_uint == OP_LOADK as libc::c_int as libc::c_uint {
                            (i >> 0i32 + 6i32 + 8i32
                                & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                                as libc::c_int
                        } else {
                            (*(*p).code.offset((pc + 1i32) as isize) >> 0i32 + 6i32
                                & !((!(0i32 as Instruction)) << 9i32 + 9i32 + 8i32) << 0i32)
                                as libc::c_int
                        };
                    if (*(*p).k.offset(b_0 as isize)).tt_ & 0xfi32 == 4i32 {
                        *name = (&mut (*((*(*p).k.offset(b_0 as isize)).value_.gc as *mut GCUnion))
                            .ts as *mut TString
                            as *mut libc::c_char)
                            .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
                        return b"constant\x00" as *const u8 as *const libc::c_char;
                    }
                }
                12 => {
                    /* key index */
                    let mut k_0: libc::c_int = (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int;
                    kname(p, pc, k_0, name);
                    return b"method\x00" as *const u8 as *const libc::c_char;
                }
                _ => {}
            }
        }
        /* go through to return NULL */
        /* could not find reasonable name */
        return 0 as *const libc::c_char;
    };
}
/*
** find a "name" for the RK value 'c'
*/
unsafe extern "C" fn kname(
    mut p: *mut Proto,
    mut pc: libc::c_int,
    mut c: libc::c_int,
    mut name: *mut *const libc::c_char,
) -> () {
    if 0 != c & 1i32 << 9i32 - 1i32 {
        /* is 'c' a constant? */
        let mut kvalue: *mut TValue =
            &mut *(*p).k.offset((c & !(1i32 << 9i32 - 1i32)) as isize) as *mut TValue;
        if (*kvalue).tt_ & 0xfi32 == 4i32 {
            /* literal constant? */
            /* it is its own name */
            *name = (&mut (*((*kvalue).value_.gc as *mut GCUnion)).ts as *mut TString
                as *mut libc::c_char)
                .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
            return;
        }
    } else {
        /* else no reasonable name found */
        /* 'c' is a register */
        /* search for 'c' */
        let mut what: *const libc::c_char = getobjname(p, pc, c, name);
        if !what.is_null() && *what as libc::c_int == 'c' as i32 {
            /* found a constant name? */
            /* 'name' already filled */
            return;
        }
    }
    /* else no reasonable name found */
    /* no reasonable name found */
    *name = b"?\x00" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn upvalname(mut p: *mut Proto, mut uv: libc::c_int) -> *const libc::c_char {
    let mut s: *mut TString = (*(*p).upvalues.offset(uv as isize)).name;
    if s.is_null() {
        return b"?\x00" as *const u8 as *const libc::c_char;
    } else {
        return (s as *mut libc::c_char)
            .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
    };
}
/*
** try to find last instruction before 'lastpc' that modified register 'reg'
*/
unsafe extern "C" fn findsetreg(
    mut p: *mut Proto,
    mut lastpc: libc::c_int,
    mut reg: libc::c_int,
) -> libc::c_int {
    let mut pc: libc::c_int = 0;
    /* keep last instruction that changed 'reg' */
    let mut setreg: libc::c_int = -1i32;
    /* any code before this address is conditional */
    let mut jmptarget: libc::c_int = 0i32;
    pc = 0i32;
    while pc < lastpc {
        let mut i: Instruction = *(*p).code.offset(pc as isize);
        let mut op: OpCode = (i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode;
        let mut a: libc::c_int =
            (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32) as libc::c_int;
        match op as libc::c_uint {
            4 => {
                let mut b: libc::c_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                    & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                    as libc::c_int;
                /* set registers from 'a' to 'a+b' */
                if a <= reg && reg <= a + b {
                    setreg = filterpc(pc, jmptarget)
                }
            }
            41 => {
                /* affect all regs above its base */
                if reg >= a + 2i32 {
                    setreg = filterpc(pc, jmptarget)
                }
            }
            36 | 37 => {
                /* affect all registers above base */
                if reg >= a {
                    setreg = filterpc(pc, jmptarget)
                }
            }
            30 => {
                let mut b_0: libc::c_int =
                    (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                        as libc::c_int - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32);
                let mut dest: libc::c_int = pc + 1i32 + b_0;
                /* jump is forward and do not skip 'lastpc'? */
                if pc < dest && dest <= lastpc {
                    if dest > jmptarget {
                        /* update 'jmptarget' */
                        jmptarget = dest
                    }
                }
            }
            _ => {
                /* any instruction that set A */
                if 0 != luaP_opmodes[op as usize] as libc::c_int & 1i32 << 6i32 && reg == a {
                    setreg = filterpc(pc, jmptarget)
                }
            }
        }
        pc += 1
    }
    return setreg;
}
unsafe extern "C" fn filterpc(mut pc: libc::c_int, mut jmptarget: libc::c_int) -> libc::c_int {
    /* is code conditional (inside a jump)? */
    if pc < jmptarget {
        /* cannot know who sets that register */
        return -1i32;
    } else {
        return pc;
    };
}
unsafe extern "C" fn currentline(mut ci: *mut CallInfo) -> libc::c_int {
    return if !(*(*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l.p)
        .lineinfo
        .is_null()
    {
        *(*(*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l.p)
            .lineinfo
            .offset(currentpc(ci) as isize)
    } else {
        -1i32
    };
}
unsafe extern "C" fn funcinfo(mut ar: *mut lua_Debug, mut cl: *mut Closure) -> () {
    let mut p: *mut Proto = 0 as *mut Proto;
    if cl.is_null() || (*cl).c.tt as libc::c_int == 6i32 | 2i32 << 4i32 {
        (*ar).source = b"=[C]\x00" as *const u8 as *const libc::c_char;
        (*ar).linedefined = -1i32;
        (*ar).lastlinedefined = -1i32;
        (*ar).what = b"C\x00" as *const u8 as *const libc::c_char
    } else {
        p = (*cl).l.p;
        (*ar).source = if !(*p).source.is_null() {
            ((*p).source as *mut libc::c_char)
                .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize)
        } else {
            b"=?\x00" as *const u8 as *const libc::c_char
        };
        (*ar).linedefined = (*p).linedefined;
        (*ar).lastlinedefined = (*p).lastlinedefined;
        (*ar).what = if (*ar).linedefined == 0i32 {
            b"main\x00" as *const u8 as *const libc::c_char
        } else {
            b"Lua\x00" as *const u8 as *const libc::c_char
        }
    }
    luaO_chunkid((*ar).short_src.as_mut_ptr(), (*ar).source, 60i32 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn lua_getlocal(
    mut L: *mut lua_State,
    mut ar: *const lua_Debug,
    mut n: libc::c_int,
) -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    swapextra(L);
    if ar.is_null() {
        /* information about non-active function? */
        /* not a Lua function? */
        if !((*(*L).top.offset(-1isize)).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32) {
            name = 0 as *const libc::c_char
        } else {
            name = luaF_getlocalname(
                (*((*(*L).top.offset(-1isize)).value_.gc as *mut GCUnion))
                    .cl
                    .l
                    .p,
                n,
                0i32,
            )
        }
    } else {
        /* active function; get information through 'ar' */
        /* to avoid warnings */
        let mut pos: StkId = 0 as StkId;
        name = findlocal(L, (*ar).i_ci, n, &mut pos);
        if !name.is_null() {
            let mut io1: *mut TValue = (*L).top;
            *io1 = *pos;
            (*L).top = (*L).top.offset(1isize)
        }
    }
    swapextra(L);
    return name;
}
unsafe extern "C" fn findlocal(
    mut L: *mut lua_State,
    mut ci: *mut CallInfo,
    mut n: libc::c_int,
    mut pos: *mut StkId,
) -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut base: StkId = 0 as *mut TValue;
    if 0 != (*ci).callstatus as libc::c_int & 1i32 << 1i32 {
        /* access to vararg values? */
        if n < 0i32 {
            return findvararg(ci, -n, pos);
        } else {
            base = (*ci).u.l.base;
            name = luaF_getlocalname(
                (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l.p,
                n,
                currentpc(ci),
            )
        }
    } else {
        base = (*ci).func.offset(1isize)
    }
    if name.is_null() {
        /* no 'standard' name? */
        let mut limit: StkId = if ci == (*L).ci {
            (*L).top
        } else {
            (*(*ci).next).func
        };
        /* is 'n' inside 'ci' stack? */
        if limit.wrapping_offset_from(base) as libc::c_long >= n as libc::c_long && n > 0i32 {
            /* generic name for any valid slot */
            name = b"(*temporary)\x00" as *const u8 as *const libc::c_char
        } else {
            return 0 as *const libc::c_char;
        }
    }
    *pos = base.offset((n - 1i32) as isize);
    return name;
}
unsafe extern "C" fn findvararg(
    mut ci: *mut CallInfo,
    mut n: libc::c_int,
    mut pos: *mut StkId,
) -> *const libc::c_char {
    let mut nparams: libc::c_int =
        (*(*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l.p).numparams as libc::c_int;
    if n >= (*ci).u.l.base.wrapping_offset_from((*ci).func) as libc::c_long as libc::c_int - nparams
    {
        /* no such vararg */
        return 0 as *const libc::c_char;
    } else {
        *pos = (*ci).func.offset(nparams as isize).offset(n as isize);
        /* generic name for any vararg */
        return b"(*vararg)\x00" as *const u8 as *const libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_setlocal(
    mut L: *mut lua_State,
    mut ar: *const lua_Debug,
    mut n: libc::c_int,
) -> *const libc::c_char {
    /* to avoid warnings */
    let mut pos: StkId = 0 as StkId;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    swapextra(L);
    name = findlocal(L, (*ar).i_ci, n, &mut pos);
    if !name.is_null() {
        let mut io1: *mut TValue = pos;
        *io1 = *(*L).top.offset(-1isize);
        /* pop value */
        (*L).top = (*L).top.offset(-1isize)
    }
    swapextra(L);
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn lua_sethook(
    mut L: *mut lua_State,
    mut func: lua_Hook,
    mut mask: libc::c_int,
    mut count: libc::c_int,
) -> () {
    if func.is_none() || mask == 0i32 {
        /* turn off hooks? */
        mask = 0i32;
        func = None
    }
    if 0 != (*(*L).ci).callstatus as libc::c_int & 1i32 << 1i32 {
        (*L).oldpc = (*(*L).ci).u.l.savedpc
    }
    ::std::ptr::write_volatile(&mut (*L).hook as *mut lua_Hook, func);
    (*L).basehookcount = count;
    (*L).hookcount = (*L).basehookcount;
    (*L).hookmask = mask as lu_byte as sig_atomic_t;
}
#[no_mangle]
pub unsafe extern "C" fn lua_gethook(mut L: *mut lua_State) -> lua_Hook {
    return (*L).hook;
}
#[no_mangle]
pub unsafe extern "C" fn lua_gethookmask(mut L: *mut lua_State) -> libc::c_int {
    return (*L).hookmask;
}
#[no_mangle]
pub unsafe extern "C" fn lua_gethookcount(mut L: *mut lua_State) -> libc::c_int {
    return (*L).basehookcount;
}
/*
** $Id: ldebug.h,v 2.14.1.1 2017/04/19 17:20:42 roberto Exp $
** Auxiliary functions from Debug Interface module
** See Copyright Notice in lua.h
*/
#[no_mangle]
pub unsafe extern "C" fn luaG_typeerror(
    mut L: *mut lua_State,
    mut o: *const TValue,
    mut op: *const libc::c_char,
) -> ! {
    let mut t: *const libc::c_char = luaT_objtypename(L, o);
    luaG_runerror(
        L,
        b"attempt to %s a %s value%s\x00" as *const u8 as *const libc::c_char,
        op,
        t,
        varinfo(L, o),
    );
}
unsafe extern "C" fn varinfo(mut L: *mut lua_State, mut o: *const TValue) -> *const libc::c_char {
    /* to avoid warnings */
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut ci: *mut CallInfo = (*L).ci;
    let mut kind: *const libc::c_char = 0 as *const libc::c_char;
    if 0 != (*ci).callstatus as libc::c_int & 1i32 << 1i32 {
        /* check whether 'o' is an upvalue */
        kind = getupvalname(ci, o, &mut name);
        /* no? try a register */
        if kind.is_null() && 0 != isinstack(ci, o) {
            kind = getobjname(
                (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l.p,
                currentpc(ci),
                o.wrapping_offset_from((*ci).u.l.base) as libc::c_long as libc::c_int,
                &mut name,
            )
        }
    }
    return if !kind.is_null() {
        luaO_pushfstring(
            L,
            b" (%s \'%s\')\x00" as *const u8 as *const libc::c_char,
            kind,
            name,
        )
    } else {
        b"\x00" as *const u8 as *const libc::c_char
    };
}
/* }====================================================== */
/*
** The subtraction of two potentially unrelated pointers is
** not ISO C, but it should not crash a program; the subsequent
** checks are ISO C and ensure a correct result.
*/
unsafe extern "C" fn isinstack(mut ci: *mut CallInfo, mut o: *const TValue) -> libc::c_int {
    let mut i: ptrdiff_t = o.wrapping_offset_from((*ci).u.l.base) as libc::c_long;
    return (0i32 as libc::c_long <= i
        && i < (*ci).top.wrapping_offset_from((*ci).u.l.base) as libc::c_long
        && (*ci).u.l.base.offset(i as isize) == o as StkId) as libc::c_int;
}
/*
** Checks whether value 'o' came from an upvalue. (That can only happen
** with instructions OP_GETTABUP/OP_SETTABUP, which operate directly on
** upvalues.)
*/
unsafe extern "C" fn getupvalname(
    mut ci: *mut CallInfo,
    mut o: *const TValue,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut c: *mut LClosure = &mut (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l;
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < (*c).nupvalues as libc::c_int {
        if (*(*c).upvals[i as usize]).v == o as *mut TValue {
            *name = upvalname((*c).p, i);
            return b"upvalue\x00" as *const u8 as *const libc::c_char;
        } else {
            i += 1
        }
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn luaG_errormsg(mut L: *mut lua_State) -> ! {
    if (*L).errfunc != 0i32 as libc::c_long {
        /* is there an error handling function? */
        let mut errfunc: StkId =
            ((*L).stack as *mut libc::c_char).offset((*L).errfunc as isize) as *mut TValue;
        /* move argument */
        let mut io1: *mut TValue = (*L).top;
        *io1 = *(*L).top.offset(-1isize);
        /* push function */
        let mut io1_0: *mut TValue = (*L).top.offset(-1isize);
        *io1_0 = *errfunc;
        /* assume EXTRA_STACK */
        (*L).top = (*L).top.offset(1isize);
        /* call it */
        luaD_callnoyield(L, (*L).top.offset(-2isize), 1i32);
    }
    luaD_throw(L, 2i32);
}
#[no_mangle]
pub unsafe extern "C" fn luaG_addinfo(
    mut L: *mut lua_State,
    mut msg: *const libc::c_char,
    mut src: *mut TString,
    mut line: libc::c_int,
) -> *const libc::c_char {
    let mut buff: [libc::c_char; 60] = [0; 60];
    if !src.is_null() {
        luaO_chunkid(
            buff.as_mut_ptr(),
            (src as *mut libc::c_char)
                .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize),
            60i32 as size_t,
        );
    } else {
        /* no source available; use "?" instead */
        buff[0usize] = '?' as i32 as libc::c_char;
        buff[1usize] = '\u{0}' as i32 as libc::c_char
    }
    return luaO_pushfstring(
        L,
        b"%s:%d: %s\x00" as *const u8 as *const libc::c_char,
        buff.as_mut_ptr(),
        line,
        msg,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaG_concaterror(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
) -> ! {
    if (*p1).tt_ & 0xfi32 == 4i32 || (*p1).tt_ & 0xfi32 == 3i32 {
        p1 = p2
    }
    luaG_typeerror(
        L,
        p1,
        b"concatenate\x00" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaG_opinterror(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut msg: *const libc::c_char,
) -> ! {
    let mut temp: lua_Number = 0.;
    /* first operand is wrong? */
    if 0 == if (*p1).tt_ == 3i32 | 0i32 << 4i32 {
        temp = (*p1).value_.n;
        1i32
    } else {
        luaV_tonumber_(p1, &mut temp)
    } {
        /* now second is wrong */
        p2 = p1
    }
    luaG_typeerror(L, p2, msg);
}
#[no_mangle]
pub unsafe extern "C" fn luaG_tointerror(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
) -> ! {
    let mut temp: lua_Integer = 0;
    if 0 == if (*p1).tt_ == 3i32 | 1i32 << 4i32 {
        temp = (*p1).value_.i;
        1i32
    } else {
        luaV_tointeger(p1, &mut temp, 0i32)
    } {
        p2 = p1
    }
    luaG_runerror(
        L,
        b"number%s has no integer representation\x00" as *const u8 as *const libc::c_char,
        varinfo(L, p2),
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaG_ordererror(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
) -> ! {
    let mut t1: *const libc::c_char = luaT_objtypename(L, p1);
    let mut t2: *const libc::c_char = luaT_objtypename(L, p2);
    if strcmp(t1, t2) == 0i32 {
        luaG_runerror(
            L,
            b"attempt to compare two %s values\x00" as *const u8 as *const libc::c_char,
            t1,
        );
    } else {
        luaG_runerror(
            L,
            b"attempt to compare %s with %s\x00" as *const u8 as *const libc::c_char,
            t1,
            t2,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaG_traceexec(mut L: *mut lua_State) -> () {
    let mut ci: *mut CallInfo = (*L).ci;
    let mut mask: lu_byte = (*L).hookmask as lu_byte;
    (*L).hookcount -= 1;
    let mut counthook: libc::c_int =
        ((*L).hookcount == 0i32 && 0 != mask as libc::c_int & 1i32 << 3i32) as libc::c_int;
    if 0 != counthook {
        /* reset count */
        (*L).hookcount = (*L).basehookcount
    } else if 0 == mask as libc::c_int & 1i32 << 2i32 {
        /* no line hook and count != 0; nothing to be done */
        return;
    }
    if 0 != (*ci).callstatus as libc::c_int & 1i32 << 6i32 {
        /* called hook last time? */
        /* erase mark */
        (*ci).callstatus = ((*ci).callstatus as libc::c_int & !(1i32 << 6i32)) as libc::c_ushort;
        /* do not call hook again (VM yielded, so it did not move) */
        return;
    } else {
        if 0 != counthook {
            /* call count hook */
            luaD_hook(L, 3i32, -1i32);
        }
        if 0 != mask as libc::c_int & 1i32 << 2i32 {
            let mut p: *mut Proto = (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l.p;
            let mut npc: libc::c_int = (*ci).u.l.savedpc.wrapping_offset_from((*p).code)
                as libc::c_long as libc::c_int - 1i32;
            let mut newline: libc::c_int = if !(*p).lineinfo.is_null() {
                *(*p).lineinfo.offset(npc as isize)
            } else {
                -1i32
            };
            /* call linehook when enter a new function, */
            if npc == 0i32 || (*ci).u.l.savedpc <= (*L).oldpc
                || newline != if !(*p).lineinfo.is_null() {
                    *(*p).lineinfo.offset(
                        ((*L).oldpc.wrapping_offset_from((*p).code) as libc::c_long as libc::c_int
                            - 1i32) as isize,
                    )
                } else {
                    -1i32
                } {
                /* when jump back (loop), or when */
                /* enter a new line */
                /* call line hook */
                luaD_hook(L, 2i32, newline);
            }
        }
        (*L).oldpc = (*ci).u.l.savedpc;
        if (*L).status as libc::c_int == 1i32 {
            /* did hook yield? */
            if 0 != counthook {
                /* undo decrement to zero */
                (*L).hookcount = 1i32
            }
            /* undo increment (resume will increment it again) */
            (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(-1isize);
            /* mark that it yielded */
            (*ci).callstatus = ((*ci).callstatus as libc::c_int | 1i32 << 6i32) as libc::c_ushort;
            /* protect stack below results */
            (*ci).func = (*L).top.offset(-1isize);
            luaD_throw(L, 1i32);
        } else {
            return;
        }
    };
}