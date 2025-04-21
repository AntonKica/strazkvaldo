<script lang="ts">
	import type { PageProps } from './$types';
    import { base } from '$app/paths';
	import { goto } from '$app/navigation';
    import { datetime_rfc3339_to_string, duration_in_seconds_to_string } from '$lib/common';

	let { data }: PageProps = $props();
	const one_time_activity = data.one_time_activity;
</script>

<h1>jednorázová aktivita {one_time_activity.code}</h1>
<b>názov</b> {one_time_activity.name} <br>
<b>typ</b> {one_time_activity.activity_type.text} <br>
<b>kritickosť</b> {one_time_activity.criticality_type.text} <br>
<b>dátum</b> {datetime_rfc3339_to_string(one_time_activity.date)} <br>
<b>trvanie</b> {duration_in_seconds_to_string(one_time_activity.duration_in_seconds)} <br>
<b>popis</b> {one_time_activity.description} <br>
<br>
<button type="button" onclick={() => goto(`${base}/user/one-time-activity/${one_time_activity.code}/edit`, { invalidateAll: true})}>Uprav jednorázovú aktivitu</button>