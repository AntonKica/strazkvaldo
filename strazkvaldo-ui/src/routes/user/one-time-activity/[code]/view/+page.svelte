<script lang="ts">
	import type { PageProps } from './$types';
	import { goto } from '$app/navigation';
    import { duration_in_seconds_to_string } from '$lib/common';
    import { UI_USER_ONE_TIME_ACTIVITY, UI_USER_ROOM, delete_entity } from '$lib/uiRoutes';
    import { SVC_USER_ONE_TIME_ACTIVITY } from '$lib/serviceRoutes';

	let { data }: PageProps = $props();
	const one_time_activity = data.one_time_activity;
</script>

<h1>jednorázová aktivita {one_time_activity.code}</h1>
<b>názov</b> {one_time_activity.name} <br>
<b>typ</b> {one_time_activity.activity_type.text} <br>
<b>kritickosť</b> {one_time_activity.criticality_type.text} <br>
<b>miestnosť</b> <a href={UI_USER_ROOM.VIEW(one_time_activity.room.code)}>{one_time_activity.room.name}</a> <br>
<b>dátum</b> {one_time_activity.due_date} <br>
<b>trvanie</b> {duration_in_seconds_to_string(one_time_activity.duration_in_seconds)} <br>

<b>popis</b> {one_time_activity.description} <br>
<br>
<button type="button" onclick={() => goto(UI_USER_ONE_TIME_ACTIVITY.EDIT(one_time_activity.code), { invalidateAll: true})}>Uprav jednorázovú aktivitu</button>
<button type="button" onclick={() => delete_entity(SVC_USER_ONE_TIME_ACTIVITY.DELETE(one_time_activity.code), UI_USER_ONE_TIME_ACTIVITY.LIST())} style="color: red;">Vymaž jednorázovú aktivitu</button>