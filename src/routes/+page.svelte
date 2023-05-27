<script lang="ts">
	import { evaluateConstant, type Constant } from "$lib/ts/types";
    import { invoke } from '@tauri-apps/api/tauri'
    import { unescapeHTML } from "$lib/ts/helper";

    let innerWidth = 0
    let innerHeight = 0
    let results: Map<string, Constant> | undefined
    let last_result: Constant | undefined
    const read_exec = (e: Event) => {
        if (e.target != null) {
            let value = (e.target as Element).innerHTML
            //set id=calc-field to innerHTML
            let calc_field = document.getElementById("calc-field") as HTMLInputElement
            calc_field.value = unescapeHTML(value)
        }
    }
    const submit = (e: Event) => {
        e.preventDefault()
        let calc_field = document.getElementById("calc-field")
        if (calc_field != null) {
            let value = (calc_field as HTMLInputElement).value
            let target = (document.getElementById("calc-select") as HTMLSelectElement).value
            invoke("calculate", { expression: value, target: target }).then((res) => {
                let r_value = res as Constant
                if (r_value == undefined) {
                    console.log(r_value)
                    return
                }
                let history_eq = document.getElementById("history-eq") as HTMLUListElement
                let li = document.createElement("li")
                li.setAttribute("class", "bg-black/75 pl-[1%] w-full rounded-l overflow-x-clip whitespace-nowrap hover:cursor-pointer")
                li.addEventListener("dblclick", read_exec)
                li.innerHTML = value
                history_eq.insertBefore(li, history_eq.firstChild)
                let history_res = document.getElementById("history-res") as HTMLUListElement
                li = document.createElement("li")
                li.setAttribute("class", "bg-black/75 pr-[1%] w-full rounded-r overflow-x-clip whitespace-nowrap hover:cursor-pointer")
                li.innerHTML = "= "+evaluateConstant(r_value)
                history_res.insertBefore(li, history_res.firstChild)
                if (results == undefined) {
                    results = new Map<string, Constant>()
                    results.set(value, r_value)
                } else if (!results.has(value)) {
                    results.set(value, r_value)
                }
                last_result = r_value
            }).catch((err) => {
                console.log(err)
            })
        }
    }
    const clear = (e: Event) => {
        e.preventDefault()
        let history_eq = document.getElementById("history-eq") as HTMLUListElement
        let history_res = document.getElementById("history-res") as HTMLUListElement
        history_eq.innerHTML = ""
        history_res.innerHTML = ""
        results = undefined
        last_result = undefined
    }
    const save = (e: Event) => {
        e.preventDefault()
        if (results == undefined) {
            return
        }
        let json = JSON.stringify(Object.fromEntries(results))
        let blob = new Blob([json], { type: "application/json" })
        let url = URL.createObjectURL(blob)
        let a = document.createElement("a")
        a.href = url
        a.download = "results.json"
        a.click()
    }
    const load = (e: Event) => {
        e.preventDefault()
        let input = document.createElement("input")
        input.type = "file"
        input.accept = "application/json"
        input.onchange = (e) => {
            if (e.target != null) {
                let file = (e.target as HTMLInputElement).files?.item(0)
                if (file != null) {
                    let reader = new FileReader()
                    reader.onload = (e) => {
                        if (e.target != null) {
                            let json = (e.target as FileReader).result as string
                            let obj = JSON.parse(json)
                            results = new Map<string, Constant>(Object.entries(obj))
                            let history_eq = document.getElementById("history-eq") as HTMLUListElement
                            let history_res = document.getElementById("history-res") as HTMLUListElement
                            history_eq.innerHTML = ""
                            history_res.innerHTML = ""
                            for (let [key, value] of results) {
                                let li = document.createElement("li")
                                li.setAttribute("class", "bg-black/75 pl-[1%] w-full rounded-l overflow-x-clip whitespace-nowrap hover:cursor-pointer")
                                li.addEventListener("dblclick", read_exec)
                                li.innerHTML = key
                                history_eq.insertBefore(li, history_eq.firstChild)
                                li = document.createElement("li")
                                li.setAttribute("class", "bg-black/75 pr-[1%] w-full rounded-r overflow-x-clip whitespace-nowrap hover:cursor-pointer")
                                li.innerHTML = "= "+evaluateConstant(value)
                                history_res.insertBefore(li, history_res.firstChild)
                            }
                        }
                    }
                    reader.readAsText(file)
                }
            }
        }
        input.click()
    }
</script>
<svelte:window bind:innerWidth bind:innerHeight />
<div class="flex flex-col gap-1 w-[3vw] fixed left-1 top-1 text-[1vw]">
    <button class="rounded bg-black/75 hover:bg-black/25 p-1" on:click={save}>
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-[2vw] h-[2vw]">
            <path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5M16.5 12L12 16.5m0 0L7.5 12m4.5 4.5V3" />
        </svg>
        Save
    </button>
    <button class="rounded bg-black/75 hover:bg-black/25 p-1" on:click={load}>
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-[2vw] h-[2vw]">
            <path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5m-13.5-9L12 3m0 0l4.5 4.5M12 3v13.5" />
          </svg>
          
        Load
    </button>
    <button class="rounded bg-black/75 hover:bg-black/25 p-1 text-red-700" on:click={clear}>
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="red" class="w-[2vw] h-[2vw]">
            <path stroke-linecap="round" stroke-linejoin="round" d="M9.75 9.75l4.5 4.5m0-4.5l-4.5 4.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        Clear
    </button>
</div>
<div class="lg:text-[1.5vw] sm:text-[3vw] text-[5vw]">
    <ul class="flex flex-row fixed left-1/2 transform -translate-x-1/2 bottom-[20vh] overflow-y-auto">
        <li>
            <ul class="rounded-l bg-stone-900/75 py-1 pl-[1.5vw] overflow-x-clip flex flex-col-reverse gap-1 w-[75vw] text-center h-[75vh]" id="history-eq">
            </ul>
        </li>
        <li>
            <ul class="rounded-r bg-stone-900/75 py-1 pr-[1.5vw] overflow-x-clip flex flex-col-reverse gap-1 w-[15vw] text-center h-[75vh]" id="history-res">
            </ul>
        </li>
    </ul>
    <!-- center horizontally -->
    <form id="calc" class="flex gap-1 fixed bottom-[12.5vh] left-1/2 transform -translate-x-1/2">
        <input
            type="text"
            placeholder="Enter an equation"
            id="calc-field"
            class="rounded overflow-x-clip w-[70vw] px-2 py-1 bg-black/40 border border-transparent hover:border-green-400"
        >
        <select id="calc-select" class="w-[9vw] rounded bg-stone-900/75 lg:text-[.5vw] sm:text-[2vw] text-[4vw] border border-transparent hover:border-green-400">
            <option value="Integer" selected>Int</option>
            <option value="Float">Float</option>
            <option value="Hexadecimal">Hex</option>
            <option value="Binary">Binary</option>
            <option value="Octal">Octal</option>
        </select>
        <input
            type="submit"
            value="{innerWidth > 640 ? 'Enter' : ">"}"
            class="rounded w-[10vw] px-2 py-1 bg-black/75 hover:bg-black/25 border border-transparent hover:border-green-400"
            on:click={submit}
        >
    </form>
    {#if last_result!=undefined }
    <span class="py-1 fixed bottom-[2vh] left-1/2 transform -translate-x-1/2 rounded bg-black/75 px-2">
        = {evaluateConstant(last_result)}
    </span>
    {/if}
</div>
