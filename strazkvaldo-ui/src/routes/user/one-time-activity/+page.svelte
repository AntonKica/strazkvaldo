<script lang="ts">
    import { base } from '$app/paths';
    import { duration_in_seconds_to_string } from '$lib/common';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<h1>Jednorázové aktivity</h1>

<a href="{base}/user/one-time-activity/create">Vytvor novú jednorázovú aktivitu &#x2795</a>
<table>
    <thead>
        <tr>
            <td>Kód</td>
            <td>Názov</td>
            <td>Typ</td>
            <td>Kritickosť</td>
            <td>Trvanie</td>
            <td>Kedy</td>
            <td></td>
        </tr>
    </thead>
    <tbody>
{#each data.one_time_activities as one_time_activity}
		<tr>
            <td>{one_time_activity.code}</td>
            <td>{one_time_activity.name}</td>
            <td>{one_time_activity.activity_type.text}</td>
            <td>{one_time_activity.criticality_type.text}</td>
            <td>{duration_in_seconds_to_string(one_time_activity.duration_in_seconds)}</td>
            <td>{one_time_activity.due_date}</td>
            <td><a href="{base}/user/one-time-activity/{one_time_activity.code}/view/">pozri</a></td>
        </tr>
{/each}
    </tbody>
</table>
<hr>
<h2> Predchádzajúce </h2>
<table>
    <thead>
        <tr>
            <td>Kód</td>
            <td>Názov</td>
            <td>Typ</td>
            <td>Kritickosť</td>
            <td>Trvanie</td>
            <td>Kedy</td>
            <td></td>
        </tr>
    </thead>
    <tbody>
{#each data.one_time_activities_expired as one_time_activity}
		<tr>
            <td>{one_time_activity.code}</td>
            <td>{one_time_activity.name}</td>
            <td>{one_time_activity.activity_type.text}</td>
            <td>{one_time_activity.criticality_type.text}</td>
            <td>{duration_in_seconds_to_string(one_time_activity.duration_in_seconds)}</td>
            <td>{one_time_activity.due_date}</td>
            <td><a href="{base}/user/one-time-activity/{one_time_activity.code}/view/">pozri</a></td>
        </tr>
{/each}
    </tbody>
</table>