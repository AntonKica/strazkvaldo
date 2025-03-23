<script lang="ts">
    import { base } from '$app/paths';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<h1>Aktivity</h1>

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
{#each data.repeatedActivities as repeatedActivity}
		<tr>
            <td>{repeatedActivity.name}</td>
            <td>{repeatedActivity.activityType}</td>
            <td>{repeatedActivity.criticalityType}</td>
                <td>
                {repeatedActivity.duration > 60 ? repeatedActivity.duration / 60 + " minút" : ""}
                {repeatedActivity.duration % 60 != 0 ? repeatedActivity.duration % 60 + " sekúnd" : ""}
             </td>
            <td>{repeatedActivity.periodicity}</td>
            <td>{repeatedActivity.startDate}</td>
            <td>{repeatedActivity.endDate}</td>
            <td>{repeatedActivity.description}</td>
            <td><a href="{base}/user/activity/view/{repeatedActivity.id}">pozri</a></td>
        </tr>
{/each}
    </tbody>
</table>

<hr>

<h2>Jednorázové aktivity</h2>
<table>
    <thead>
        <tr>
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
{#each data.oneTimeActivities as oneTimeActivity}
		<tr>
            <td>{oneTimeActivity.name}</td>
            <td>{oneTimeActivity.activityType}</td>
            <td>{oneTimeActivity.criticalityType}</td>
            <td>
                {oneTimeActivity.duration > 60 ? oneTimeActivity.duration / 60 + " minút" : ""}
                {oneTimeActivity.duration % 60 != 0 ? oneTimeActivity.duration % 60 + " sekúnd" : ""}
             </td>
            <td>{oneTimeActivity.date}</td>
            <td>{oneTimeActivity.description}</td>
            <td><a href="{base}/user/activity/view/{oneTimeActivity.id}">pozri</a></td>
        </tr>
{/each}
    </tbody>
</table>