<script lang="ts">
  import Block from "./lib/Block.svelte";
  import Calculate from "./lib/Calculate.svelte";
  import Input from "./lib/Input.svelte";
  import { invoke } from '@tauri-apps/api/tauri'

  let expression = ""
  let keyboardClickMap : Record<string,() => void> = {}
  function add(value:number | string){
    return function(){
      expression += value
    }
  }
  function add_key(value: number | string){
    expression += value
  }
  function clear(){
    expression = ""
  }
  async function calculate(){
    const exp = expression
    const result = (await invoke('parse_number', {expression: exp})) as string
    expression = result.toString()

  }
  function pop(){
    expression = expression.substring(0, expression.length - 1);
  }

  function onKeyDown(event : KeyboardEvent) {
    switch(event.key){
      case '=': {
        calculate()
        keyboardClickMap['=']()
        return
      }
      case 'Enter': {
        calculate()
        keyboardClickMap['=']()
        return
      }
      case 'Backspace': {
        pop()
        keyboardClickMap['⬅']()
        return
      }
      case 'Escape': {
        clear()
        keyboardClickMap['CE']()
        return
      }
      default: {
        console.log(event.key)
        if(event.key === '+' || event.key === '-' || event.key === '/' || event.key ===  '*' || event.key === ' '){
          add_key(event.key)
          keyboardClickMap[event.key]()
          return
        }
        
          const number = parseInt(event.key)
          if(isNaN(number)){
            return
          }      
          add_key(number)
          keyboardClickMap[event.key]()
        
        return
      }
    }
  }
</script>

<svelte:window on:keydown={onKeyDown}/>

<main class="bg-black" >
  <p class="text-xl animation">Epic Calculator</p>
  <div class="grid grid-cols-4 gap-0 w-screen h-screen" >
    <Input value={expression}  />
    <Block bind:keyboardClick={keyboardClickMap['%']} click={add('%')} content={'%'}/>
    <Block bind:keyboardClick={keyboardClickMap['C']} click={clear} content={'C'}/>
    <Block bind:keyboardClick={keyboardClickMap['CE']} click={clear} content={'CE'}/>
    <Block bind:keyboardClick={keyboardClickMap['⬅']} click={pop} content={'⬅'}/>
    <Block bind:keyboardClick={keyboardClickMap['1']} click={add(1)} content={1}/>
    <Block bind:keyboardClick={keyboardClickMap['2']} click={add(2)} content={2}/>
    <Block bind:keyboardClick={keyboardClickMap['3']} click={add(3)} content={3}/>
    <Block bind:keyboardClick={keyboardClickMap['/']} click={add("/")} content={'/'}/>
    <Block bind:keyboardClick={keyboardClickMap['4']} click={add(4)} content={4}/>
    <Block bind:keyboardClick={keyboardClickMap['5']} click={add(5)} content={5}/>
    <Block bind:keyboardClick={keyboardClickMap['6']} click={add(6)} content={6}/>
    <Block bind:keyboardClick={keyboardClickMap['*']} click={add("*")} content={'*'}/>
    <Block bind:keyboardClick={keyboardClickMap['7']} click={add(7)} content={7}/>
    <Block bind:keyboardClick={keyboardClickMap['8']} click={add(8)} content={8}/>
    <Block bind:keyboardClick={keyboardClickMap['9']} click={add(9)} content={9}/>
    <Block bind:keyboardClick={keyboardClickMap['-']} click={add("-")} content={'-'}/>
    <Block bind:keyboardClick={keyboardClickMap['0']} click={add(0)} content={0}/>
    <Block bind:keyboardClick={keyboardClickMap[',']} click={add('.')} content={','}/>
    <Block bind:keyboardClick={keyboardClickMap['+']} click={add("+")} content={'+'}/>
    <Block bind:keyboardClick={keyboardClickMap['=']} click={calculate} content={'='}/>
  </div>
</main>
<style>
  @keyframes colorChange {
  0% {
    color: red;
  }
  50% {
    color: green;
  }
  100% {
    color: blue;
  }
}

.animation {
  animation: colorChange 5s linear infinite;
}
</style>