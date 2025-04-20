<script lang="ts">
    import { base } from '$app/paths';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<h1>Jednorázové aktivity</h1>

<!--
<h2>Opakované aktivity</h2>
<table>
    <thead>
        <tr>
            <td>Názov</td>
            <td>Typ</td>
            <td>Kritickosť</td>
            <td>Trvanie</td>
            <td>Opakovanie</td>
            <td>Od</td>
            <td>Do</td>
            <td>Popis</td>
            <td></td>
        </tr>
    </thead>
    <tbody>
{#each data.repeated_activities as repeated_activity}
		<tr>
            <td>{repeated_activity.name}</td>
            <td>{repeated_activity.activity_type}</td>
            <td>{repeated_activity.criticality_type}</td>
                <td>
                {repeated_activity.duration_in_seconds > 60 ? (repeated_activity.duration_in_seconds / 60 >> 0) + " minút" : ""}
                {repeated_activity.duration_in_seconds % 60 != 0 ? (repeated_activity.duration_in_seconds % 60 >> 0) + " sekúnd" : ""}
             </td>
            <td>{repeated_activity.periodicity}</td>
            <td>{repeated_activity.startDate}</td>
            <td>{repeated_activity.endDate}</td>
            <td>{repeated_activity.description}</td>
            <td><a href="{base}/user/one-time-activity/view/{repeated_activity.id}">pozri</a></td>
        </tr>
{/each}
    </tbody>
</table>
<hr>
-->

<table>
    <thead>
        <tr>
            <td>Kód</td>
            <td>Názov</td>
            <td>Typ</td>
            <td>Kritickosť</td>
            <td>Trvanie</td>
            <td>Kedy</td>
            <td>Popis</td>
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
            <td>
                {one_time_activity.duration_in_seconds > 60 ? (one_time_activity.duration_in_seconds / 60 >> 0) + " minút" : ""}
                {one_time_activity.duration_in_seconds % 60 != 0 ? (one_time_activity.duration_in_seconds % 60 >> 0) + " sekúnd" : ""}
             </td>
            <td>{one_time_activity.date}</td>
            <td>{one_time_activity.description}</td>
            <td><a href="{base}/user/one-time-activity/view/{one_time_activity.code}">pozri</a></td>
        </tr>
{/each}
    </tbody>
</table>