<script lang="ts">
	import type { PageProps } from './$types';
    import { base } from '$app/paths';
	import { goto } from '$app/navigation';
    import { datetime_rfc3339_to_string, duration_in_seconds_to_string } from '$lib/common';
    import { UI_USER_REPEATED_ACTIVITY } from '$lib/uiRoutes';

	let { data }: PageProps = $props();
	const repeated_activity = data.repeated_activity;
</script>

<h1>jednorázová aktivita {repeated_activity.code}</h1>
<b>názov</b> {repeated_activity.name} <br>
<b>typ</b> {repeated_activity.activity_type.text} <br>
<b>kritickosť</b> {repeated_activity.criticality_type.text} <br>
<b>dátum od</b> {datetime_rfc3339_to_string(repeated_activity.start_date)} <br>
<b>dátum do</b> {datetime_rfc3339_to_string(repeated_activity.end_date)} <br>
<b>trvanie</b> {duration_in_seconds_to_string(repeated_activity.duration_in_seconds)} <br>
<b>popis</b> {repeated_activity.description} <br>
<br>
<button type="button" onclick={() => goto(UI_USER_REPEATED_ACTIVITY.EDIT(repeated_activity.code), { invalidateAll: true})}>Uprav jednorázovú aktivitu</button>