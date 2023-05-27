<script lang="ts">
	import { evaluateConstant, type Constant } from "$lib/ts/types";
    import { invoke } from '@tauri-apps/api/tauri'
    import { unescapeHTML } from "$lib/ts/helper";

    let innerWidth = 0
    let innerHeight = 0
    let results: Constant | undefined = undefined
    const read_exec = (e: Event) => {
        console.log(e)
        if (e.target != null) {
            let value = (e.target as Element).innerHTML
            console.log(unescapeHTML(value))
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
                results = res as Constant
                if (results == undefined) {
                    console.log(results)
                    return
                }
                let history = document.getElementById("history") as HTMLUListElement
                let li = document.createElement("li")
                li.setAttribute("class", "bg-black/75 px-[1%] w-full rounded overflow-x-clip whitespace-nowrap hover:cursor-pointer")
                li.addEventListener("dblclick", read_exec)
                li.innerHTML = value!
                history.insertBefore(li, history.firstChild)
            }).catch((err) => {
                console.log(err)
            })
        }
    }
</script>
<svelte:window bind:innerWidth bind:innerHeight />
<div class="lg:text-[1.5vw] sm:text-[3vw] text-[5vw]">
    <ul class="rounded bg-stone-900/75 py-1 px-[1.5vw] overflow-y-auto overflow-x-clip flex flex-col-reverse gap-1 w-[90vw] text-center fixed bottom-[20vh] left-1/2 transform -translate-x-1/2 h-[75vh]" id="history">
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
    {#if results != undefined }
    <span class="fixed bottom-[2vh] left-1/2 transform -translate-x-1/2 rounded bg-black/75 px-1">
        = {evaluateConstant(results)}
    </span>
    {/if}
</div>
