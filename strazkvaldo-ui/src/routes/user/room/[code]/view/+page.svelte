<script lang="ts">
	import type { PageProps } from './$types';
	import { goto } from '$app/navigation';
    import { UI_USER_ROOM, delete_entity } from '$lib/uiRoutes';
    import { SVC_USER_ROOM } from '$lib/serviceRoutes';
	
	let { data }: PageProps = $props();
	let room = data.room;
</script>

<h1>Miestnosť</h1>
<b>kód</b> {room.code} <br>
<b>názov</b> {room.name} <br>
<b>typ</b> {room.room_type.text} <br>
<b>popis</b> {room.description} <br>
<button type="button" onclick={() => goto(UI_USER_ROOM.EDIT(room.code), { invalidateAll: true})}>Uprav miestnosť</button>
<button type="button" onclick={() => delete_entity(SVC_USER_ROOM.DELETE(room.code), UI_USER_ROOM.LIST())} style="color: red;">Vymaž miestnosť</button>