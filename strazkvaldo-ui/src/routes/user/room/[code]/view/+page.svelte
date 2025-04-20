<script lang="ts">
	import type { PageProps } from './$types';
    import { base } from '$app/paths';
	import { goto } from '$app/navigation';
	
	let { data }: PageProps = $props();
</script>

<h1>Aktivita</h1>
<b>kód</b> {data.room.code} <br>
<b>názov</b> {data.room.name} <br>
<b>typ</b> {data.room.room_type} <br>
<b>popis</b> {data.room.description} <br>
<button type="button" onclick={() => goto(`${base}/user/room/${data.room.code}/edit`, { invalidateAll: true})}>Uprav miestnosť</button>