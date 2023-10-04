<script lang="ts">
    export let value: string
</script>

<div class="h-100 w-100 bg-gray-800  col-span-4 " >
    <p class="text-6xl text-white mt-20">{value.length === 0 ? "Masukan Angka" : value}</p>
</div>

