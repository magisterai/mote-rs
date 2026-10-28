# 7 – Incompatibilities with the Previous Version

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](028-debug-getlocal-thread-level-local.md)

<a id="7"></a>

# 7 –  Incompatibilities with the Previous Version

Here we list the incompatibilities that you may find when moving a program
from Lua 5.0 to Lua 5.1.
You can avoid most of the incompatibilities compiling Lua with
appropriate options (see file `luaconf.h`).
However,
all these compatibility options will be removed in the next version of Lua.

<a id="7.1"></a>

## 7.1 –  Changes in the Language

- The vararg system changed from the pseudo-argument `arg` with a
  table with the extra arguments to the vararg expression.
  (See compile-time option `LUA_COMPAT_VARARG` in `luaconf.h`.)
- There was a subtle change in the scope of the implicit
  variables of the **for** statement and for the **repeat** statement.
- The long string/long comment syntax (`[[string]]`)
  does not allow nesting.
  You can use the new syntax (`[=[string]=]`) in these cases.
  (See compile-time option `LUA_COMPAT_LSTR` in `luaconf.h`.)

<a id="7.2"></a>

## 7.2 –  Changes in the Libraries

- Function `string.gfind` was renamed [`string.gmatch`](023-package-path.md#pdf-string.gmatch).
  (See compile-time option `LUA_COMPAT_GFIND` in `luaconf.h`.)
- When [`string.gsub`](023-package-path.md#pdf-string.gsub) is called with a function as its
  third argument,
  whenever this function returns **nil** or **false** the
  replacement string is the whole match,
  instead of the empty string.
- Function `table.setn` was deprecated.
  Function `table.getn` corresponds
  to the new length operator (`#`);
  use the operator instead of the function.
  (See compile-time option `LUA_COMPAT_GETN` in `luaconf.h`.)
- Function `loadlib` was renamed [`package.loadlib`](022-module-name.md#pdf-package.loadlib).
  (See compile-time option `LUA_COMPAT_LOADLIB` in `luaconf.h`.)
- Function `math.mod` was renamed [`math.fmod`](025-table-insert-table-pos-value.md#pdf-math.fmod).
  (See compile-time option `LUA_COMPAT_MOD` in `luaconf.h`.)
- Functions `table.foreach` and `table.foreachi` are deprecated.
  You can use a for loop with `pairs` or `ipairs` instead.
- There were substantial changes in function [`require`](022-module-name.md#pdf-require) due to
  the new module system.
  However, the new behavior is mostly compatible with the old,
  but `require` gets the path from [`package.path`](023-package-path.md#pdf-package.path) instead
  of from `LUA_PATH`.
- Function [`collectgarbage`](020-collectgarbage-opt-arg.md#pdf-collectgarbage) has different arguments.
  Function `gcinfo` is deprecated;
  use `collectgarbage("count")` instead.

<a id="7.3"></a>

## 7.3 –  Changes in the API

- The `luaopen_*` functions (to open libraries)
  cannot be called directly,
  like a regular C function.
  They must be called through Lua,
  like a Lua function.
- Function `lua_open` was replaced by [`lua_newstate`](011-lua-istable.md#lua_newstate) to
  allow the user to set a memory-allocation function.
  You can use [`luaL_newstate`](018-lual-error.md#luaL_newstate) from the standard library to
  create a state with a standard allocation function
  (based on `realloc`).
- Functions `luaL_getn` and `luaL_setn`
  (from the auxiliary library) are deprecated.
  Use [`lua_objlen`](011-lua-istable.md#lua_objlen) instead of `luaL_getn`
  and nothing instead of `luaL_setn`.
- Function `luaL_openlib` was replaced by [`luaL_register`](019-lual-pushresult.md#luaL_register).
- Function `luaL_checkudata` now throws an error when the given value
  is not a userdata of the expected type.
  (In Lua 5.0 it returned `NULL`.)

<a id="8"></a>

# 8 –  The Complete Syntax of Lua

Here is the complete syntax of Lua in extended BNF.
(It does not describe operator precedences.)



```


	chunk ::= {stat [`;´]} [laststat [`;´]]

	block ::= chunk

	stat ::=  varlist `=´ explist | 
		 functioncall | 
		 do block end | 
		 while exp do block end | 
		 repeat block until exp | 
		 if exp then block {elseif exp then block} [else block] end | 
		 for Name `=´ exp `,´ exp [`,´ exp] do block end | 
		 for namelist in explist do block end | 
		 function funcname funcbody | 
		 local function Name funcbody | 
		 local namelist [`=´ explist] 

	laststat ::= return [explist] | break

	funcname ::= Name {`.´ Name} [`:´ Name]

	varlist ::= var {`,´ var}

	var ::=  Name | prefixexp `[´ exp `]´ | prefixexp `.´ Name 

	namelist ::= Name {`,´ Name}

	explist ::= {exp `,´} exp

	exp ::=  nil | false | true | Number | String | `...´ | function | 
		 prefixexp | tableconstructor | exp binop exp | unop exp 

	prefixexp ::= var | functioncall | `(´ exp `)´

	functioncall ::=  prefixexp args | prefixexp `:´ Name args 

	args ::=  `(´ [explist] `)´ | tableconstructor | String 

	function ::= function funcbody

	funcbody ::= `(´ [parlist] `)´ block end

	parlist ::= namelist [`,´ `...´] | `...´

	tableconstructor ::= `{´ [fieldlist] `}´

	fieldlist ::= field {fieldsep field} [fieldsep]

	field ::= `[´ exp `]´ `=´ exp | Name `=´ exp | exp

	fieldsep ::= `,´ | `;´

	binop ::= `+´ | `-´ | `*´ | `/´ | `^´ | `%´ | `..´ | 
		 `<´ | `<=´ | `>´ | `>=´ | `==´ | `~=´ | 
		 and | or

	unop ::= `-´ | not | `#´

```




Last update:
Thu Aug 29 20:43:58 UTC 2019
