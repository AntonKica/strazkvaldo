<script lang="ts">
    import { base } from '$app/paths';
    import { duration_in_seconds_to_string, datetime_rfc3339_to_string } from '$lib/common';
    import { UI_USER_REPEATED_ACTIVITY } from '$lib/uiRoutes';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
    const repeated_activities = data.repeated_activities;
</script>

<h1>Opakované aktivity</h1>

<a href={UI_USER_REPEATED_ACTIVITY.CREATE()}>Vytvor novú opakovanú aktivitu &#x2795</a>
<table>
    <thead>
        <tr>
            <td>Názov</td>
            <td>Typ aktivity</td>
            <td>Kritickosť</td>
            <td>Trvanie</td>
            <td>Opakovanie</td>
            <td>Od</td>
            <td>Do</td>
            <td></td>
        </tr>
    </thead>
    <tbody>
{#each data.repeated_activities as repeated_activity}
		<tr>
            <td>{repeated_activity.name}</td>
            <td>{repeated_activity.activity_type.text}</td>
            <td>{repeated_activity.criticality_type.text}</td>
            <td>{duration_in_seconds_to_string(repeated_activity.duration_in_seconds)}</td>
            <td>{repeated_activity.periodicity.text}</td>
            <td>{datetime_rfc3339_to_string(repeated_activity.start_date)}</td>
            <td>{datetime_rfc3339_to_string(repeated_activity.end_date)}</td>
            <td><a href={UI_USER_REPEATED_ACTIVITY.VIEW(repeated_activity.code)}>pozri</a></td>
        </tr>
{/each}
    </tbody>
</table>