/* tslint:disable */
import * as wasm from './wasm_game_of_life_bg';

const __wbg_log_55d0b01f9e295146_target = console.log;

let cachedDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbg_log_55d0b01f9e295146(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    __wbg_log_55d0b01f9e295146_target(varg0);
}
/**
*/
export const Cell = Object.freeze({ Dead:0,Alive:1, });

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null) {
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    }
    return cachedGlobalArgumentPtr;
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error('expected a number argument');
}

const __wbg_random_91b6ba2fda260d29_target = Math.random.bind(Math) || function() {
    throw new Error(`wasm-bindgen: Math.random.bind(Math) does not exist`);
};

export function __wbg_random_91b6ba2fda260d29() {
    return __wbg_random_91b6ba2fda260d29_target();
}

class ConstructorToken {
    constructor(ptr) {
        this.ptr = ptr;
    }
}

function freeUniverse(ptr) {
    
    wasm.__wbg_universe_free(ptr);
}
/**
*/
export class Universe {
    
    static __construct(ptr) {
        return new Universe(new ConstructorToken(ptr));
    }
    
    constructor(...args) {
        if (args.length === 1 && args[0] instanceof ConstructorToken) {
            this.ptr = args[0].ptr;
            return;
        }
        throw new Error('you cannot invoke `new` directly without having a method annotated a constructor');
    }
    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeUniverse(ptr);
    }
    /**
    * @returns {void}
    */
    tick() {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        return wasm.universe_tick(this.ptr);
    }
    /**
    * @returns {Universe}
    */
    static new() {
        return Universe.__construct(wasm.universe_new());
    }
    /**
    * @returns {string}
    */
    render() {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        const retptr = globalArgumentPtr();
        wasm.universe_render(retptr, this.ptr);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];
        
        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;
        
    }
    /**
    * @returns {number}
    */
    width() {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        return wasm.universe_width(this.ptr);
    }
    /**
    * @returns {number}
    */
    height() {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        return wasm.universe_height(this.ptr);
    }
    /**
    * @returns {number}
    */
    cells() {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        return wasm.universe_cells(this.ptr);
    }
    /**
    * @param {number} arg0
    * @param {number} arg1
    * @returns {void}
    */
    toggle_cell(arg0, arg1) {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        _assertNum(arg0);
        _assertNum(arg1);
        return wasm.universe_toggle_cell(this.ptr, arg0, arg1);
    }
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

