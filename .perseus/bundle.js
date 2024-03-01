import{create_client as a,sign_in as b,get_session as c,fetch_schedule as d}from"./snippets/aaeo-website-819d6a467bb31bdd/src/supabase.js";let wasm;const heap=new Array(128).fill(undefined);heap.push(undefined,null,true,false);function getObject(e){return heap[e]}let WASM_VECTOR_LEN=0;let cachedUint8Memory0=null;function getUint8Memory0(){if(cachedUint8Memory0===null||cachedUint8Memory0.byteLength===0){cachedUint8Memory0=new Uint8Array(wasm.memory.buffer)};return cachedUint8Memory0}const cachedTextEncoder=typeof TextEncoder!=='undefined'?new TextEncoder('utf-8'):{encode:()=>{throw Error('TextEncoder not available')}};const encodeString=typeof cachedTextEncoder.encodeInto==='function'?function(e,f){return cachedTextEncoder.encodeInto(e,f)}:function(e,f){const g=cachedTextEncoder.encode(e);f.set(g);return {read:e.length,written:g.length}};function passStringToWasm0(e,f,g){if(g===undefined){const l=cachedTextEncoder.encode(e);const m=f(l.length,1)>>>0;getUint8Memory0().subarray(m,m+ l.length).set(l);WASM_VECTOR_LEN=l.length;return m};let h=e.length;let i=f(h,1)>>>0;const j=getUint8Memory0();let k=0;for(;k<h;k++){const l=e.charCodeAt(k);if(l>0x7F)break;j[i+ k]=l};if(k!==h){if(k!==0){e=e.slice(k)};i=g(i,h,h=k+ e.length*3,1)>>>0;const l=getUint8Memory0().subarray(i+ k,i+ h);const m=encodeString(e,l);k+=m.written};WASM_VECTOR_LEN=k;return i}function isLikeNone(e){return e===undefined||e===null}let cachedInt32Memory0=null;function getInt32Memory0(){if(cachedInt32Memory0===null||cachedInt32Memory0.byteLength===0){cachedInt32Memory0=new Int32Array(wasm.memory.buffer)};return cachedInt32Memory0}let heap_next=heap.length;function dropObject(e){if(e<132)return;heap[e]=heap_next;heap_next=e}function takeObject(e){const f=getObject(e);dropObject(e);return f}function addHeapObject(e){if(heap_next===heap.length)heap.push(heap.length+ 1);const f=heap_next;heap_next=heap[f];heap[f]=e;return f}const cachedTextDecoder=typeof TextDecoder!=='undefined'?new TextDecoder('utf-8',{ignoreBOM:true,fatal:true}):{decode:()=>{throw Error('TextDecoder not available')}};if(typeof TextDecoder!=='undefined'){cachedTextDecoder.decode()};function getStringFromWasm0(e,f){e=e>>>0;return cachedTextDecoder.decode(getUint8Memory0().subarray(e,e+ f))}let cachedFloat64Memory0=null;function getFloat64Memory0(){if(cachedFloat64Memory0===null||cachedFloat64Memory0.byteLength===0){cachedFloat64Memory0=new Float64Array(wasm.memory.buffer)};return cachedFloat64Memory0}function debugString(e){const f=typeof e;if(f=='number'||f=='boolean'||e==null){return `${e}`};if(f=='string'){return `"${e}"`};if(f=='symbol'){const i=e.description;if(i==null){return 'Symbol'}else{return `Symbol(${i})`}};if(f=='function'){const i=e.name;if(typeof i=='string'&&i.length>0){return `Function(${i})`}else{return 'Function'}};if(Array.isArray(e)){const i=e.length;let j='[';if(i>0){j+=debugString(e[0])};for(let k=1;k<i;k++){j+=', '+ debugString(e[k])};j+=']';return j};const g=/\[object ([^\]]+)\]/.exec(toString.call(e));let h;if(g.length>1){h=g[1]}else{return toString.call(e)};if(h=='Object'){try{return 'Object('+ JSON.stringify(e)+ ')'}catch(i){return 'Object'}};if(e instanceof Error){return `${e.name}: ${e.message}\n${e.stack}`};return h}function makeClosure(e,f,g,h){const i={a:e,b:f,cnt:1,dtor:g};const j=(...k)=>{i.cnt++;try{return h(i.a,i.b,...k)}finally{if(--i.cnt===0){wasm.__wbindgen_export_2.get(i.dtor)(i.a,i.b);i.a=0}}};j.original=i;return j}function __wbg_adapter_34(e,f){wasm._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__ha455ac767b506555(e,f)}function makeMutClosure(e,f,g,h){const i={a:e,b:f,cnt:1,dtor:g};const j=(...k)=>{i.cnt++;const l=i.a;i.a=0;try{return h(l,i.b,...k)}finally{if(--i.cnt===0){wasm.__wbindgen_export_2.get(i.dtor)(l,i.b)}else{i.a=l}}};j.original=i;return j}function __wbg_adapter_37(e,f,g){wasm.wasm_bindgen__convert__closures__invoke1_mut__h1b9205145591493e(e,f,addHeapObject(g))}function __wbg_adapter_40(e,f){wasm.wasm_bindgen__convert__closures__invoke0_mut__h9abb707f454620b6(e,f)}function __wbg_adapter_43(e,f,g){wasm.wasm_bindgen__convert__closures__invoke1_mut__h86124378aca4778b(e,f,addHeapObject(g))}function getCachedStringFromWasm0(e,f){if(e===0){return getObject(f)}else{return getStringFromWasm0(e,f)}}function handleError(e,f){try{return e.apply(this,f)}catch(g){wasm.__wbindgen_exn_store(addHeapObject(g))}}async function __wbg_load(e,f){if(typeof Response==='function'&&e instanceof Response){if(typeof WebAssembly.instantiateStreaming==='function'){try{return await WebAssembly.instantiateStreaming(e,f)}catch(h){if(e.headers.get('Content-Type')!='application/wasm'){console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",h)}else{throw h}}};const g=await e.arrayBuffer();return await WebAssembly.instantiate(g,f)}else{const g=await WebAssembly.instantiate(e,f);if(g instanceof WebAssembly.Instance){return {instance:g,module:e}}else{return g}}}function __wbg_get_imports(){const e={};e.wbg={};e.wbg.__wbindgen_string_get=function(f,g){const j=getObject(g);const k=typeof j==='string'?j:undefined;var h=isLikeNone(k)?0:passStringToWasm0(k,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);var i=WASM_VECTOR_LEN;getInt32Memory0()[f/4+ 1]=i;getInt32Memory0()[f/4+ 0]=h};e.wbg.__wbg_createclient_370b528b7363840c=function(){return handleError(function(f,g,h,i){var j=getCachedStringFromWasm0(f,g);var k=getCachedStringFromWasm0(h,i);a(j,k)},arguments)};e.wbg.__wbindgen_object_drop_ref=function(f){takeObject(f)};e.wbg.__wbindgen_object_clone_ref=function(f){const g=getObject(f);return addHeapObject(g)};e.wbg.__wbindgen_string_new=function(f,g){const h=getStringFromWasm0(f,g);return addHeapObject(h)};e.wbg.__wbg_getsession_bdc13d4571ca4e29=function(){return handleError(function(){const f=c();return addHeapObject(f)},arguments)};e.wbg.__wbg_fetchschedule_94e334014299767e=function(){return handleError(function(){const f=d();return addHeapObject(f)},arguments)};e.wbg.__wbindgen_is_object=function(f){const g=getObject(f);const h=typeof g==='object'&&g!==null;return h};e.wbg.__wbindgen_is_undefined=function(f){const g=getObject(f)===undefined;return g};e.wbg.__wbindgen_in=function(f,g){const h=getObject(f) in getObject(g);return h};e.wbg.__wbg_signin_7cd9d3fb71b4195d=function(){return handleError(function(f,g){var h=getCachedStringFromWasm0(f,g);const i=b(h);return addHeapObject(i)},arguments)};e.wbg.__wbindgen_number_get=function(f,g){const h=getObject(g);const i=typeof h==='number'?h:undefined;getFloat64Memory0()[f/8+ 1]=isLikeNone(i)?0:i;getInt32Memory0()[f/4+ 0]=!isLikeNone(i)};e.wbg.__wbindgen_jsval_loose_eq=function(f,g){const h=getObject(f)==getObject(g);return h};e.wbg.__wbindgen_boolean_get=function(f){const g=getObject(f);const h=typeof g==='boolean'?(g?1:0):2;return h};e.wbg.__wbindgen_error_new=function(f,g){const h=new Error(getStringFromWasm0(f,g));return addHeapObject(h)};e.wbg.__wbg_getwithrefkey_5e6d9547403deab8=function(f,g){const h=getObject(f)[getObject(g)];return addHeapObject(h)};e.wbg.__wbindgen_cb_drop=function(f){const g=takeObject(f).original;if(g.cnt--==1){g.a=0;return true};const h=false;return h};e.wbg.__wbindgen_jsval_eq=function(f,g){const h=getObject(f)===getObject(g);return h};e.wbg.__wbg_nodeId_bbf0efafa303e805=function(f,g){const h=getObject(g).$$$nodeId;getInt32Memory0()[f/4+ 1]=isLikeNone(h)?0:h;getInt32Memory0()[f/4+ 0]=!isLikeNone(h)};e.wbg.__wbg_setnodeId_433ef8ed15bd1612=function(f,g){getObject(f).$$$nodeId=g>>>0};e.wbg.__wbg_setclassName_f86a95d6ffe042e6=function(){return handleError(function(f,g,h){var i=getCachedStringFromWasm0(g,h);getObject(f).className=i},arguments)};e.wbg.__wbg_queueMicrotask_b580a35152f7cc7c=function(f){queueMicrotask(getObject(f))};e.wbg.__wbg_remove_179d3d8cd04e3f10=function(f){getObject(f).remove()};e.wbg.__wbg_newwitheventinitdict_4e9208842a1bd356=function(){return handleError(function(f,g,h){var i=getCachedStringFromWasm0(f,g);const j=new CustomEvent(i,getObject(h));return addHeapObject(j)},arguments)};e.wbg.__wbg_documentElement_0df873a7503a9af9=function(f){const g=getObject(f).documentElement;return isLikeNone(g)?0:addHeapObject(g)};e.wbg.__wbg_body_674aec4c1c0910cd=function(f){const g=getObject(f).body;return isLikeNone(g)?0:addHeapObject(g)};e.wbg.__wbg_createComment_6b5ea2660a7c961a=function(f,g,h){var i=getCachedStringFromWasm0(g,h);const j=getObject(f).createComment(i);return addHeapObject(j)};e.wbg.__wbg_createElement_4891554b28d3388b=function(){return handleError(function(f,g,h){var i=getCachedStringFromWasm0(g,h);const j=getObject(f).createElement(i);return addHeapObject(j)},arguments)};e.wbg.__wbg_createTextNode_2fd22cd7e543f938=function(f,g,h){var i=getCachedStringFromWasm0(g,h);const j=getObject(f).createTextNode(i);return addHeapObject(j)};e.wbg.__wbg_getElementById_cc0e0d931b0d9a28=function(f,g,h){var i=getCachedStringFromWasm0(g,h);const j=getObject(f).getElementById(i);return isLikeNone(j)?0:addHeapObject(j)};e.wbg.__wbg_querySelector_52ded52c20e23921=function(){return handleError(function(f,g,h){var i=getCachedStringFromWasm0(g,h);const j=getObject(f).querySelector(i);return isLikeNone(j)?0:addHeapObject(j)},arguments)};e.wbg.__wbg_querySelectorAll_c03e8664a5a0f0c5=function(){return handleError(function(f,g,h){var i=getCachedStringFromWasm0(g,h);const j=getObject(f).querySelectorAll(i);return addHeapObject(j)},arguments)};e.wbg.__wbg_tagName_5d48e5a3ca410385=function(f,g){const h=getObject(g).tagName;const i=passStringToWasm0(h,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const j=WASM_VECTOR_LEN;getInt32Memory0()[f/4+ 1]=j;getInt32Memory0()[f/4+ 0]=i};e.wbg.__wbg_setid_1984ee27e5075311=function(f,g,h){var i=getCachedStringFromWasm0(g,h);getObject(f).id=i};e.wbg.__wbg_innerHTML_7957d4fb76221e5a=function(f,g){const h=getObject(g).innerHTML;const i=passStringToWasm0(h,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const j=WASM_VECTOR_LEN;getInt32Memory0()[f/4+ 1]=j;getInt32Memory0()[f/4+ 0]=i};e.wbg.__wbg_setinnerHTML_b089587252408b67=function(f,g,h){var i=getCachedStringFromWasm0(g,h);getObject(f).innerHTML=i};e.wbg.__wbg_outerHTML_f7749ceff37b5832=function(f,g){const h=getObject(g).outerHTML;const i=passStringToWasm0(h,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const j=WASM_VECTOR_LEN;getInt32Memory0()[f/4+ 1]=j;getInt32Memory0()[f/4+ 0]=i};e.wbg.__wbg_closest_48a6f505535d402f=function(){return handleError(function(f,g,h){var i=getCachedStringFromWasm0(g,h);const j=getObject(f).closest(i);return isLikeNone(j)?0:addHeapObject(j)},arguments)};e.wbg.__wbg_getAttribute_3d8fcc9eaea35a17=function(f,g,h,i){var j=getCachedStringFromWasm0(h,i);const m=getObject(g).getAttribute(j);var k=isLikeNone(m)?0:passStringToWasm0(m,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);var l=WASM_VECTOR_LEN;getInt32Memory0()[f/4+ 1]=l;getInt32Memory0()[f/4+ 0]=k};e.wbg.__wbg_insertAdjacentHTML_04bc2b21165e1256=function(){return handleError(function(f,g,h,i,j){var k=getCachedStringFromWasm0(g,h);var l=getCachedStringFromWasm0(i,j);getObject(f).insertAdjacentHTML(k,l)},arguments)};e.wbg.__wbg_querySelectorAll_521f18edab19a2d0=function(){return handleError(function(f,g,h){var i=getCachedStringFromWasm0(g,h);const j=getObject(f).querySelectorAll(i);return addHeapObject(j)},arguments)};e.wbg.__wbg_removeAttribute_d8404da431968808=function(){return handleError(function(f,g,h){var i=getCachedStringFromWasm0(g,h);getObject(f).removeAttribute(i)},arguments)};e.wbg.__wbg_setAttribute_e7e80b478b7b8b2f=function(){return handleError(function(f,g,h,i,j){var k=getCachedStringFromWasm0(g,h);var l=getCachedStringFromWasm0(i,j);getObject(f).setAttribute(k,l)},arguments)};e.wbg.__wbg_append_184b4e731e022bf0=function(){return handleError(function(f,g){getObject(f).append(getObject(g))},arguments)};e.wbg.__wbg_target_f171e89c61e2bccf=function(f){const g=getObject(f).target;return isLikeNone(g)?0:addHeapObject(g)};e.wbg.__wbg_preventDefault_24104f3f0a54546a=function(f){getObject(f).preventDefault()};e.wbg.__wbg_addEventListener_5651108fc3ffeb6e=function(){return handleError(function(f,g,h,i){var j=getCachedStringFromWasm0(g,h);getObject(f).addEventListener(j,getObject(i))},arguments)};e.wbg.__wbg_dispatchEvent_a622a6455be582eb=function(){return handleError(function(f,g){const h=getObject(f).dispatchEvent(getObject(g));return h},arguments)};e.wbg.__wbg_pushState_1145414a47c0b629=function(){return handleError(function(f,g,h,i,j,k){var l=getCachedStringFromWasm0(h,i);var m=getCachedStringFromWasm0(j,k);getObject(f).pushState(getObject(g),l,m)},arguments)};e.wbg.__wbg_replaceState_2e530b05e604adc4=function(){return handleError(function(f,g,h,i,j,k){var l=getCachedStringFromWasm0(h,i);var m=getCachedStringFromWasm0(j,k);getObject(f).replaceState(getObject(g),l,m)},arguments)};e.wbg.__wbg_rel_7a5daa1634871543=function(f,g){const h=getObject(g).rel;const i=passStringToWasm0(h,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const j=WASM_VECTOR_LEN;getInt32Memory0()[f/4+ 1]=j;getInt32Memory0()[f/4+ 0]=i};e.wbg.__wbg_origin_8d3cc36142304e88=function(f,g){const h=getObject(g).origin;const i=passStringToWasm0(h,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const j=WASM_VECTOR_LEN;getInt32Memory0()[f/4+ 1]=j;getInt32Memory0()[f/4+ 0]=i};e.wbg.__wbg_pathname_28ec40e7c9ba7ea6=function(f,g){const h=getObject(g).pathname;const i=passStringToWasm0(h,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const j=WASM_VECTOR_LEN;getInt32Memory0()[f/4+ 1]=j;getInt32Memory0()[f/4+ 0]=i};e.wbg.__wbg_hash_1d8a5443664f6e12=function(f,g){const h=getObject(g).hash;const i=passStringToWasm0(h,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const j=WASM_VECTOR_LEN;getInt32Memory0()[f/4+ 1]=j;getInt32Memory0()[f/4+ 0]=i};e.wbg.__wbg_href_47b90f0ddf3ddcd7=function(f,g){const h=getObject(g).href;const i=passStringToWasm0(h,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const j=WASM_VECTOR_LEN;getInt32Memory0()[f/4+ 1]=j;getInt32Memory0()[f/4+ 0]=i};e.wbg.__wbg_altKey_612289acf855835c=function(f){const g=getObject(f).altKey;return g};e.wbg.__wbg_ctrlKey_582686fb2263dd3c=function(f){const g=getObject(f).ctrlKey;return g};e.wbg.__wbg_shiftKey_48e8701355d8e2d4=function(f){const g=getObject(f).shiftKey;return g};e.wbg.__wbg_metaKey_43193b7cc99f8914=function(f){const g=getObject(f).metaKey;return g};e.wbg.__wbg_origin_50aa482fa6784a0a=function(){return handleError(function(f,g){const h=getObject(g).origin;const i=passStringToWasm0(h,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const j=WASM_VECTOR_LEN;getInt32Memory0()[f/4+ 1]=j;getInt32Memory0()[f/4+ 0]=i},arguments)};e.wbg.__wbg_pathname_c8fd5c498079312d=function(){return handleError(function(f,g){const h=getObject(g).pathname;const i=passStringToWasm0(h,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const j=WASM_VECTOR_LEN;getInt32Memory0()[f/4+ 1]=j;getInt32Memory0()[f/4+ 0]=i},arguments)};e.wbg.__wbg_search_6c3c472e076ee010=function(){return handleError(function(f,g){const h=getObject(g).search;const i=passStringToWasm0(h,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const j=WASM_VECTOR_LEN;getInt32Memory0()[f/4+ 1]=j;getInt32Memory0()[f/4+ 0]=i},arguments)};e.wbg.__wbg_hash_a1a795b89dda8e3d=function(){return handleError(function(f,g){const h=getObject(g).hash;const i=passStringToWasm0(h,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const j=WASM_VECTOR_LEN;getInt32Memory0()[f/4+ 1]=j;getInt32Memory0()[f/4+ 0]=i},arguments)};e.wbg.__wbg_replace_5d1d2b7956cafd7b=function(){return handleError(function(f,g,h){var i=getCachedStringFromWasm0(g,h);getObject(f).replace(i)},arguments)};e.wbg.__wbg_language_4f943dbdc59c3951=function(f,g){const j=getObject(g).language;var h=isLikeNone(j)?0:passStringToWasm0(j,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);var i=WASM_VECTOR_LEN;getInt32Memory0()[f/4+ 1]=i;getInt32Memory0()[f/4+ 0]=h};e.wbg.__wbg_languages_4ab80469955a57f7=function(f){const g=getObject(f).languages;return addHeapObject(g)};e.wbg.__wbg_nodeType_238f049908daf027=function(f){const g=getObject(f).nodeType;return g};e.wbg.__wbg_parentNode_9e53f8b17eb98c9d=function(f){const g=getObject(f).parentNode;return isLikeNone(g)?0:addHeapObject(g)};e.wbg.__wbg_childNodes_64dab37cf9d252dd=function(f){const g=getObject(f).childNodes;return addHeapObject(g)};e.wbg.__wbg_nextSibling_304d9aac7c2774ae=function(f){const g=getObject(f).nextSibling;return isLikeNone(g)?0:addHeapObject(g)};e.wbg.__wbg_nodeValue_c263dd6af29e6efb=function(f,g){const j=getObject(g).nodeValue;var h=isLikeNone(j)?0:passStringToWasm0(j,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);var i=WASM_VECTOR_LEN;getInt32Memory0()[f/4+ 1]=i;getInt32Memory0()[f/4+ 0]=h};e.wbg.__wbg_settextContent_28d80502cf08bde7=function(f,g,h){var i=getCachedStringFromWasm0(g,h);getObject(f).textContent=i};e.wbg.__wbg_appendChild_51339d4cde00ee22=function(){return handleError(function(f,g){const h=getObject(f).appendChild(getObject(g));return addHeapObject(h)},arguments)};e.wbg.__wbg_insertBefore_ffa01d4b747c95fc=function(){return handleError(function(f,g,h){const i=getObject(f).insertBefore(getObject(g),getObject(h));return addHeapObject(i)},arguments)};e.wbg.__wbg_removeChild_973429f368206138=function(){return handleError(function(f,g){const h=getObject(f).removeChild(getObject(g));return addHeapObject(h)},arguments)};e.wbg.__wbg_replaceChild_3ec13b15218637aa=function(){return handleError(function(f,g,h){const i=getObject(f).replaceChild(getObject(g),getObject(h));return addHeapObject(i)},arguments)};e.wbg.__wbg_length_7aeee1534dbcb390=function(f){const g=getObject(f).length;return g};e.wbg.__wbg_get_c77649dd3862b63a=function(f,g){const h=getObject(f)[g>>>0];return isLikeNone(h)?0:addHeapObject(h)};e.wbg.__wbg_newwithstrandinit_cad5cd6038c7ff5d=function(){return handleError(function(f,g,h){var i=getCachedStringFromWasm0(f,g);const j=new Request(i,getObject(h));return addHeapObject(j)},arguments)};e.wbg.__wbg_instanceof_Response_fc4327dbfcdf5ced=function(f){let g;try{g=getObject(f) instanceof Response}catch{g=false};const h=g;return h};e.wbg.__wbg_status_ac85a3142a84caa2=function(f){const g=getObject(f).status;return g};e.wbg.__wbg_text_a667ac1770538491=function(){return handleError(function(f){const g=getObject(f).text();return addHeapObject(g)},arguments)};e.wbg.__wbg_pathname_57290e07c6bc0683=function(f,g){const h=getObject(g).pathname;const i=passStringToWasm0(h,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const j=WASM_VECTOR_LEN;getInt32Memory0()[f/4+ 1]=j;getInt32Memory0()[f/4+ 0]=i};e.wbg.__wbg_new_a76f6bcb38f791ea=function(){return handleError(function(f,g){var h=getCachedStringFromWasm0(f,g);const i=new URL(h);return addHeapObject(i)},arguments)};e.wbg.__wbg_newwithstr_ad74b2c1808b94ca=function(){return handleError(function(f,g){var h=getCachedStringFromWasm0(f,g);const i=new URLSearchParams(h);return addHeapObject(i)},arguments)};e.wbg.__wbg_get_1207388de9966926=function(f,g,h,i){var j=getCachedStringFromWasm0(h,i);const m=getObject(g).get(j);var k=isLikeNone(m)?0:passStringToWasm0(m,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);var l=WASM_VECTOR_LEN;getInt32Memory0()[f/4+ 1]=l;getInt32Memory0()[f/4+ 0]=k};e.wbg.__wbg_instanceof_Window_9029196b662bc42a=function(f){let g;try{g=getObject(f) instanceof Window}catch{g=false};const h=g;return h};e.wbg.__wbg_document_f7ace2b956f30a4f=function(f){const g=getObject(f).document;return isLikeNone(g)?0:addHeapObject(g)};e.wbg.__wbg_location_56243dba507f472d=function(f){const g=getObject(f).location;return addHeapObject(g)};e.wbg.__wbg_history_3c2280e6b2a9316e=function(){return handleError(function(f){const g=getObject(f).history;return addHeapObject(g)},arguments)};e.wbg.__wbg_navigator_7c9103698acde322=function(f){const g=getObject(f).navigator;return addHeapObject(g)};e.wbg.__wbg_scrollTo_3fa406312438ebdf=function(f,g,h){getObject(f).scrollTo(g,h)};e.wbg.__wbg_get_cb7c1c2da725c920=function(f,g,h){var i=getCachedStringFromWasm0(g,h);const j=getObject(f)[i];return isLikeNone(j)?0:addHeapObject(j)};e.wbg.__wbg_fetch_336b6f0cb426b46e=function(f,g){const h=getObject(f).fetch(getObject(g));return addHeapObject(h)};e.wbg.__wbg_log_1d3ae0273d8f4f8a=function(f){console.log(getObject(f))};e.wbg.__wbg_self_1ff1d729e9aae938=function(){return handleError(function(){const f=self.self;return addHeapObject(f)},arguments)};e.wbg.__wbg_window_5f4faef6c12b79ec=function(){return handleError(function(){const f=window.window;return addHeapObject(f)},arguments)};e.wbg.__wbg_globalThis_1d39714405582d3c=function(){return handleError(function(){const f=globalThis.globalThis;return addHeapObject(f)},arguments)};e.wbg.__wbg_global_651f05c6a0944d1c=function(){return handleError(function(){const f=global.global;return addHeapObject(f)},arguments)};e.wbg.__wbg_length_fff51ee6522a1a18=function(f){const g=getObject(f).length;return g};e.wbg.__wbg_new_b51585de1b234aff=function(){const f=new Object();return addHeapObject(f)};e.wbg.__wbg_decodeURIComponent_3822776b09f2f835=function(){return handleError(function(f,g){var h=getCachedStringFromWasm0(f,g);const i=decodeURIComponent(h);return addHeapObject(i)},arguments)};e.wbg.__wbg_get_44be0491f933a435=function(f,g){const h=getObject(f)[g>>>0];return addHeapObject(h)};e.wbg.__wbg_instanceof_ArrayBuffer_39ac22089b74fddb=function(f){let g;try{g=getObject(f) instanceof ArrayBuffer}catch{g=false};const h=g;return h};e.wbg.__wbg_newnoargs_581967eacc0e2604=function(f,g){var h=getCachedStringFromWasm0(f,g);const i=new Function(h);return addHeapObject(i)};e.wbg.__wbg_call_cb65541d95d71282=function(){return handleError(function(f,g){const h=getObject(f).call(getObject(g));return addHeapObject(h)},arguments)};e.wbg.__wbg_is_205d914af04a8faa=function(f,g){const h=Object.is(getObject(f),getObject(g));return h};e.wbg.__wbg_get_97b561fb56f034b5=function(){return handleError(function(f,g){const h=Reflect.get(getObject(f),getObject(g));return addHeapObject(h)},arguments)};e.wbg.__wbg_set_092e06b0f9d71865=function(){return handleError(function(f,g,h){const i=Reflect.set(getObject(f),getObject(g),getObject(h));return i},arguments)};e.wbg.__wbg_buffer_085ec1f694018c4f=function(f){const g=getObject(f).buffer;return addHeapObject(g)};e.wbg.__wbg_resolve_53698b95aaf7fcf8=function(f){const g=Promise.resolve(getObject(f));return addHeapObject(g)};e.wbg.__wbg_then_f7e06ee3c11698eb=function(f,g){const h=getObject(f).then(getObject(g));return addHeapObject(h)};e.wbg.__wbg_then_b2267541e2a73865=function(f,g,h){const i=getObject(f).then(getObject(g),getObject(h));return addHeapObject(i)};e.wbg.__wbg_new_8125e318e6245eed=function(f){const g=new Uint8Array(getObject(f));return addHeapObject(g)};e.wbg.__wbg_instanceof_Uint8Array_d8d9cb2b8e8ac1d4=function(f){let g;try{g=getObject(f) instanceof Uint8Array}catch{g=false};const h=g;return h};e.wbg.__wbg_length_72e2208bbc0efc61=function(f){const g=getObject(f).length;return g};e.wbg.__wbg_set_5cf90238115182c3=function(f,g,h){getObject(f).set(getObject(g),h>>>0)};e.wbg.__wbindgen_debug_string=function(f,g){const h=debugString(getObject(g));const i=passStringToWasm0(h,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);const j=WASM_VECTOR_LEN;getInt32Memory0()[f/4+ 1]=j;getInt32Memory0()[f/4+ 0]=i};e.wbg.__wbindgen_throw=function(f,g){throw new Error(getStringFromWasm0(f,g))};e.wbg.__wbindgen_memory=function(){const f=wasm.memory;return addHeapObject(f)};e.wbg.__wbindgen_closure_wrapper791=function(f,g,h){const i=makeClosure(f,g,237,__wbg_adapter_34);return addHeapObject(i)};e.wbg.__wbindgen_closure_wrapper799=function(f,g,h){const i=makeMutClosure(f,g,237,__wbg_adapter_37);return addHeapObject(i)};e.wbg.__wbindgen_closure_wrapper1618=function(f,g,h){const i=makeMutClosure(f,g,362,__wbg_adapter_40);return addHeapObject(i)};e.wbg.__wbindgen_closure_wrapper2686=function(f,g,h){const i=makeMutClosure(f,g,398,__wbg_adapter_43);return addHeapObject(i)};return e}function __wbg_init_memory(e,f){}function __wbg_finalize_init(e,f){wasm=e.exports;__wbg_init.__wbindgen_wasm_module=f;cachedFloat64Memory0=null;cachedInt32Memory0=null;cachedUint8Memory0=null;wasm.__wbindgen_start();return wasm}function initSync(e){if(wasm!==undefined)return wasm;const f=__wbg_get_imports();__wbg_init_memory(f);if(!(e instanceof WebAssembly.Module)){e=new WebAssembly.Module(e)};const g=new WebAssembly.Instance(e,f);return __wbg_finalize_init(g,e)}async function __wbg_init(e){if(wasm!==undefined)return wasm;if(typeof e==='undefined'){e=new URL('perseus_engine_bg.wasm',import.meta.url)};const f=__wbg_get_imports();if(typeof e==='string'||typeof Request==='function'&&e instanceof Request||typeof URL==='function'&&e instanceof URL){e=fetch(e)};__wbg_init_memory(f);const {instance:g,module:h}=await __wbg_load(await e,f);return __wbg_finalize_init(g,h)}export default __wbg_init;;