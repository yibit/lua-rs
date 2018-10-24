use types::*;

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
    /*
     ** Lua Upvalues
     */
    pub type UpVal;
    /*
     ** 'module' operation for hashing (size is always a power of 2)
     */
    /*
     ** (address of) a fixed nil value
     */
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    fn luaS_new(L: *mut lua_State, str: *const lua_char) -> *mut TString;
    #[no_mangle]
    fn luaH_getshortstr(t: *mut Table, key: *mut TString) -> *const TValue;
    /*
     ** $Id: lgc.h,v 2.91.1.1 2017/04/19 17:39:34 roberto Exp $
     ** Garbage Collector
     ** See Copyright Notice in lua.h
     */
    /*
     ** Collectable objects may have one of three colors: white, which
     ** means the object is not marked; gray, which means the
     ** object is marked, but its references may be not marked; and
     ** black, which means that the object and all its references are marked.
     ** The main invariant of the garbage collector, while marking objects,
     ** is that a black object can never point to a white one. Moreover,
     ** any gray object must be in a "gray list" (gray, grayagain, weak,
     ** allweak, ephemeron) so that it can be visited again before finishing
     ** the collection cycle. These lists have no meaning when the invariant
     ** is not being enforced (e.g., sweep phase).
     */
    /* how much to allocate before next GC step */
    /* ~100 small strings */
    /*
     ** Possible states of the Garbage Collector
     */
    /*
     ** macro to tell when main invariant (white objects cannot point to black
     ** ones) must be kept. During a collection, the sweep
     ** phase may break the invariant, as objects turned white may point to
     ** still-black objects. The invariant is restored when sweep ends and
     ** all objects are white again.
     */
    /*
     ** some useful bit tricks
     */
    /* Layout for bit use in 'marked' field: */
    /* object is white (type 0) */
    /* object is white (type 1) */
    /* object is black */
    /* object has been marked for finalization */
    /* bit 7 is currently used by tests (luaL_checkmemory) */
    /* neither white nor black */
    /*
     ** Does one step of collection when debt becomes positive. 'pre'/'pos'
     ** allows some adjustments to be done only when needed. macro
     ** 'condchangemem' is used only for heavy tests (forcing a full
     ** GC cycle on every opportunity)
     */
    /* more often than not, 'pre'/'pos' are empty */
    #[no_mangle]
    fn luaC_fix(L: *mut lua_State, o: *mut GCObject) -> ();
    #[no_mangle]
    fn luaD_callnoyield(L: *mut lua_State, func: StkId, nResults: lua_int) -> ();
    #[no_mangle]
    fn luaD_call(L: *mut lua_State, func: StkId, nResults: lua_int) -> ();
    #[no_mangle]
    fn luaG_opinterror(
        L: *mut lua_State,
        p1: *const TValue,
        p2: *const TValue,
        msg: *const lua_char,
    ) -> !;
    #[no_mangle]
    fn luaG_tointerror(L: *mut lua_State, p1: *const TValue, p2: *const TValue) -> !;
    #[no_mangle]
    fn luaV_tonumber_(obj: *const TValue, n: *mut lua_Number) -> lua_int;
    #[no_mangle]
    fn luaG_concaterror(L: *mut lua_State, p1: *const TValue, p2: *const TValue) -> !;
}

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
    pub nci: lua_ushort,
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
    pub stacksize: lua_int,
    pub basehookcount: lua_int,
    pub hookcount: lua_int,
    pub nny: lua_ushort,
    pub nCcalls: lua_ushort,
    pub hookmask: sig_atomic_t,
    pub allowhook: lu_byte,
}
/* 16-bit ints */
/* }{ */
/* } */
/* chars used as small naturals (so that 'char' is reserved for characters) */
pub type lu_byte = lua_uchar;
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
    pub event: lua_int,
    pub name: *const lua_char,
    pub namewhat: *const lua_char,
    pub what: *const lua_char,
    pub source: *const lua_char,
    pub currentline: lua_int,
    pub linedefined: lua_int,
    pub lastlinedefined: lua_int,
    pub nups: lua_uchar,
    pub nparams: lua_uchar,
    pub isvararg: lua_char,
    pub istailcall: lua_char,
    pub short_src: [lua_char; 60],
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
    pub nresults: lua_short,
    pub callstatus: lua_ushort,
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
    Option<unsafe extern "C" fn(_: *mut lua_State, _: lua_int, _: lua_KContext) -> lua_int>;
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
pub type Instruction = lua_uint;
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
    pub tt_: lua_int,
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
    pub p: *mut lua_void,
    pub b: lua_int,
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
pub type lua_Number = lua_double;
/* type for integer functions */
pub type lua_Integer = lua_longlong;
/*
** Type for C functions registered with Lua
*/
pub type lua_CFunction = Option<unsafe extern "C" fn(_: *mut lua_State) -> lua_int>;
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
    pub ud: *mut lua_void,
    pub totalbytes: l_mem,
    pub GCdebt: l_mem,
    pub GCmemtrav: lu_mem,
    pub GCestimate: lu_mem,
    pub strt: stringtable,
    pub l_registry: TValue,
    pub seed: lua_uint,
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
    pub gcfinnum: lua_uint,
    pub gcpause: lua_int,
    pub gcstepmul: lua_int,
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
    pub hash: lua_uint,
    pub u: unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_2 {
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
    pub sizearray: lua_uint,
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
    pub nk: unnamed_3,
    pub tvk: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_3 {
    pub value_: Value,
    pub tt_: lua_int,
    pub next: lua_int,
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
    pub nuse: lua_int,
    pub size: lua_int,
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
pub type l_mem = ptrdiff_t;
/*
** Type for memory-allocation functions
*/
pub type lua_Alloc = Option<
    unsafe extern "C" fn(_: *mut lua_void, _: *mut lua_void, _: size_t, _: size_t) -> *mut lua_void,
>;
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
    pub u: lua_double,
    pub s: *mut lua_void,
    pub i: lua_Integer,
    pub l: lua_long,
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
    pub startpc: lua_int,
    pub endpc: lua_int,
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
    pub sizeupvalues: lua_int,
    pub sizek: lua_int,
    pub sizecode: lua_int,
    pub sizelineinfo: lua_int,
    pub sizep: lua_int,
    pub sizelocvars: lua_int,
    pub linedefined: lua_int,
    pub lastlinedefined: lua_int,
    pub k: *mut TValue,
    pub code: *mut Instruction,
    pub p: *mut *mut Proto,
    pub lineinfo: *mut lua_int,
    pub locvars: *mut LocVar,
    pub upvalues: *mut Upvaldesc,
    pub cache: *mut LClosure,
    pub source: *mut TString,
    pub gclist: *mut GCObject,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union Closure {
    pub c: CClosure,
    pub l: LClosure,
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
pub type TMS = lua_uint;
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
#[no_mangle]
pub static mut luaT_typenames_: [*const lua_char; 11] = unsafe {
    [
        s!(b"no value\x00"),
        s!(b"nil\x00"),
        s!(b"boolean\x00"),
        udatatypename.as_ptr(),
        s!(b"number\x00"),
        s!(b"string\x00"),
        s!(b"table\x00"),
        s!(b"function\x00"),
        udatatypename.as_ptr(),
        s!(b"thread\x00"),
        s!(b"proto\x00"),
    ]
};
/*
** $Id: ltm.c,v 2.38.1.1 2017/04/19 17:39:34 roberto Exp $
** Tag methods
** See Copyright Notice in lua.h
*/
static mut udatatypename: [lua_char; 9] = [117, 115, 101, 114, 100, 97, 116, 97, 0];
#[no_mangle]
pub unsafe extern "C" fn luaT_objtypename(
    mut L: *mut lua_State,
    mut o: *const TValue,
) -> *const lua_char {
    let mut mt: *mut Table = 0 as *mut Table;
    if (*o).tt_ == 5i32 | 1i32 << 6i32 && {
        mt = (*((*o).value_.gc as *mut GCUnion)).h.metatable;
        !mt.is_null()
    } || (*o).tt_ == 7i32 | 1i32 << 6i32 && {
        mt = (*((*o).value_.gc as *mut GCUnion)).u.metatable;
        !mt.is_null()
    } {
        let mut name: *const TValue = luaH_getshortstr(mt, luaS_new(L, s!(b"__name\x00")));
        /* is '__name' a string? */
        if (*name).tt_ & 0xfi32 == 4i32 {
            /* use it as type name */
            return (&mut (*((*name).value_.gc as *mut GCUnion)).ts as *mut TString
                as *mut lua_char)
                .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize);
        }
    }
    /* else use standard type name */
    return luaT_typenames_[(((*o).tt_ & 0xfi32) + 1i32) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn luaT_gettm(
    mut events: *mut Table,
    mut event: TMS,
    mut ename: *mut TString,
) -> *const TValue {
    let mut tm: *const TValue = luaH_getshortstr(events, ename);
    if (*tm).tt_ == 0i32 {
        /* no tag method? */
        /* cache this fact */
        (*events).flags = ((*events).flags as lua_int
            | (1u32 << event as lua_uint) as lu_byte as lua_int)
            as lu_byte;
        return 0 as *const TValue;
    } else {
        return tm;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaT_gettmbyobj(
    mut L: *mut lua_State,
    mut o: *const TValue,
    mut event: TMS,
) -> *const TValue {
    let mut mt: *mut Table = 0 as *mut Table;
    match (*o).tt_ & 0xfi32 {
        5 => mt = (*((*o).value_.gc as *mut GCUnion)).h.metatable,
        7 => mt = (*((*o).value_.gc as *mut GCUnion)).u.metatable,
        _ => mt = (*(*L).l_G).mt[((*o).tt_ & 0xfi32) as usize],
    }
    return if !mt.is_null() {
        luaH_getshortstr(mt, (*(*L).l_G).tmname[event as usize])
    } else {
        &luaO_nilobject_
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaT_init(mut L: *mut lua_State) -> () {
    /* ORDER TM */
    static mut luaT_eventname: [*const lua_char; 24] = [
        s!(b"__index\x00"),
        s!(b"__newindex\x00"),
        s!(b"__gc\x00"),
        s!(b"__mode\x00"),
        s!(b"__len\x00"),
        s!(b"__eq\x00"),
        s!(b"__add\x00"),
        s!(b"__sub\x00"),
        s!(b"__mul\x00"),
        s!(b"__mod\x00"),
        s!(b"__pow\x00"),
        s!(b"__div\x00"),
        s!(b"__idiv\x00"),
        s!(b"__band\x00"),
        s!(b"__bor\x00"),
        s!(b"__bxor\x00"),
        s!(b"__shl\x00"),
        s!(b"__shr\x00"),
        s!(b"__unm\x00"),
        s!(b"__bnot\x00"),
        s!(b"__lt\x00"),
        s!(b"__le\x00"),
        s!(b"__concat\x00"),
        s!(b"__call\x00"),
    ];
    let mut i: lua_int = 0;
    i = 0i32;
    while i < TM_N as lua_int {
        (*(*L).l_G).tmname[i as usize] = luaS_new(L, luaT_eventname[i as usize]);
        /* never collect these names */
        luaC_fix(
            L,
            &mut (*((*(*L).l_G).tmname[i as usize] as *mut GCUnion)).gc,
        );
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaT_callTM(
    mut L: *mut lua_State,
    mut f: *const TValue,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut p3: *mut TValue,
    mut hasres: lua_int,
) -> () {
    let mut io1_2: *mut TValue = 0 as *mut TValue;
    let mut result: ptrdiff_t =
        (p3 as *mut lua_char).wrapping_offset_from((*L).stack as *mut lua_char) as lua_long;
    let mut func: StkId = (*L).top;
    /* push function (assume EXTRA_STACK) */
    let mut io1: *mut TValue = func;
    *io1 = *f;
    /* 1st argument */
    let mut io1_0: *mut TValue = func.offset(1isize);
    *io1_0 = *p1;
    /* 2nd argument */
    let mut io1_1: *mut TValue = func.offset(2isize);
    *io1_1 = *p2;
    (*L).top = (*L).top.offset(3isize);
    /* no result? 'p3' is third argument */
    if 0 == hasres {
        /* 3rd argument */
        let fresh0 = (*L).top;
        (*L).top = (*L).top.offset(1);
        io1_2 = fresh0;
        *io1_2 = *p3
    }
    /* metamethod may yield only when called from Lua code */
    if 0 != (*(*L).ci).callstatus as lua_int & 1i32 << 1i32 {
        luaD_call(L, func, hasres);
    } else {
        luaD_callnoyield(L, func, hasres);
    }
    if 0 != hasres {
        /* if has result, move it to its place */
        p3 = ((*L).stack as *mut lua_char).offset(result as isize) as *mut TValue;
        let mut io1_3: *mut TValue = p3;
        (*L).top = (*L).top.offset(-1isize);
        *io1_3 = *(*L).top
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaT_callbinTM(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut res: StkId,
    mut event: TMS,
) -> lua_int {
    /* try first operand */
    let mut tm: *const TValue = luaT_gettmbyobj(L, p1, event);
    if (*tm).tt_ == 0i32 {
        /* try second operand */
        tm = luaT_gettmbyobj(L, p2, event)
    }
    if (*tm).tt_ == 0i32 {
        return 0i32;
    } else {
        luaT_callTM(L, tm, p1, p2, res, 1i32);
        return 1i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaT_trybinTM(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut res: StkId,
    mut event: TMS,
) -> () {
    if 0 == luaT_callbinTM(L, p1, p2, res, event) {
        match event as lua_uint {
            22 => {
                luaG_concaterror(L, p1, p2);
            }
            13 | 14 | 15 | 16 | 17 | 19 => {
                let mut dummy: lua_Number = 0.;
                if 0 != if (*p1).tt_ == 3i32 | 0i32 << 4i32 {
                    dummy = (*p1).value_.n;
                    1i32
                } else {
                    luaV_tonumber_(p1, &mut dummy)
                } && 0
                    != if (*p2).tt_ == 3i32 | 0i32 << 4i32 {
                        dummy = (*p2).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(p2, &mut dummy)
                    } {
                    luaG_tointerror(L, p1, p2);
                } else {
                    luaG_opinterror(L, p1, p2, s!(b"perform bitwise operation on\x00"));
                }
            }
            _ => {
                luaG_opinterror(L, p1, p2, s!(b"perform arithmetic on\x00"));
            }
        }
    } else {
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaT_callorderTM(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut event: TMS,
) -> lua_int {
    if 0 == luaT_callbinTM(L, p1, p2, (*L).top, event) {
        /* no metamethod */
        return -1i32;
    } else {
        return !((*(*L).top).tt_ == 0i32 || (*(*L).top).tt_ == 1i32 && (*(*L).top).value_.b == 0i32)
            as lua_int;
    };
}
