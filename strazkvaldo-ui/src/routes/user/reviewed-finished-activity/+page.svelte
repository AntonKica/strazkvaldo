<script lang="ts">
    import { base } from '$app/paths';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
    let reviewed_finished_activities = data.reviewed_finished_activities;
</script>

<h1>Dokončené aktivity</h1>

<table>
    <thead>
        <tr>
            <td>kód</td>
            <td>datum</td>
            <td>popis</td>
        </tr>
    </thead>
    <tbody>
{#each reviewed_finished_activities as reviewed_finished_activity}
		<tr>
            <td>{reviewed_finished_activity.code}</td>
            <td>{reviewed_finished_activity.due_date}</td>
            <td>{reviewed_finished_activity.description}</td>
        </tr>
{/each}
    </tbody>
</table>