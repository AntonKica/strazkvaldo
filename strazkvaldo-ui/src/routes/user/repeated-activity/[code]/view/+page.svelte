<script lang="ts">
	import type { PageProps } from './$types';
	import { goto } from '$app/navigation';
    import { UI_USER_REPEATED_ACTIVITY, UI_USER_ROOM, delete_entity } from '$lib/uiRoutes';
    import { SVC_USER_REPEATED_ACTIVITY } from '$lib/serviceRoutes';
    import { day_of_week_to_string, duration_in_seconds_to_string } from '$lib/common';

	let { data }: PageProps = $props();
	const repeated_activity = data.repeated_activity;
</script>

<h1>opakovaná aktivita {repeated_activity.code}</h1>
<b>názov</b> {repeated_activity.name} <br>
<b>typ</b> {repeated_activity.activity_type.text} <br>
<b>kritickosť</b> {repeated_activity.criticality_type.text} <br>
<b>miestnosť</b> <a href={UI_USER_ROOM.VIEW(repeated_activity.room.code)}>{repeated_activity.room.name}</a> <br>
<b>trvanie</b> {duration_in_seconds_to_string(repeated_activity.duration_in_seconds)} <br>
<b>opakovanie</b> 
{#if repeated_activity.periodicity.code ==="Day" }
denne
{:else if repeated_activity.periodicity.code ==="Week"}
{day_of_week_to_string(repeated_activity.periodicity_unit)}
 {:else if repeated_activity.periodicity.code ==="Month"}
{repeated_activity.periodicity_unit}. deň v mesiaci
	 {:else if repeated_activity.periodicity.code ==="Year"}
{repeated_activity.periodicity_unit}. deň v roku
 {/if} <br>
<b>popis</b> {repeated_activity.description} <br>
<br>
<button type="button" onclick={() => goto(UI_USER_REPEATED_ACTIVITY.EDIT(repeated_activity.code), { invalidateAll: true})}>Uprav aktivitu</button>
<button type="button" onclick={() => delete_entity(SVC_USER_REPEATED_ACTIVITY.DELETE(repeated_activity.code), UI_USER_REPEATED_ACTIVITY.LIST())} style="color: red;">Vymaž aktivitu</button>