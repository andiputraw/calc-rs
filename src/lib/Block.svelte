<script lang="ts">
    export let content : string | number = undefined
    export let element: HTMLButtonElement = undefined
    export let click = () => {}
    export const keyboardClick = () => {
        if(!element.classList.contains('bg-gray-600')){
            element.classList.add('bg-gray-600')
            setTimeout(() => {
                element.classList.remove('bg-gray-600')         
            },100)
        }
    }  
</script>


<button bind:this={element} on:click={click} aria-describedby="button" class="flex justify-center border-solid border-2 rounded-lg border-gray-600 align-items-center w-100 h-100 bg-gray-800 hover:bg-gray-600">
    <div class="mx-auto my-auto text-white text-xl" >{content}</div>
</button>
