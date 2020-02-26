async function doit() {
    main = document.getElementById('main')
    wa = await WebAssembly.instantiateStreaming(fetch("main.wasm"))
    ins = wa.instance
    ex = ins.exports
    a = []
    for (i = 1; i <= 20; i++) a.push(ex.fib(i))
    main.innerText = a.join()
}
doit()