<script lang="ts">
	import type { PageProps } from './$types';
    import { UI_USER_ONE_TIME_ACTIVITY, UI_USER_REPEATED_ACTIVITY } from '$lib/uiRoutes';

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
            <td>aktivita</td>
        </tr>
    </thead>
    <tbody>
{#each reviewed_finished_activities as reviewed_finished_activity}
		<tr>
            <td>{reviewed_finished_activity.code}</td>
            <td>{reviewed_finished_activity.due_date}</td>
            <td>{reviewed_finished_activity.description}</td>
            <td>
                {#if reviewed_finished_activity.one_time_activity_code}
                <a href={UI_USER_ONE_TIME_ACTIVITY.VIEW(reviewed_finished_activity.one_time_activity_code)}>{reviewed_finished_activity.one_time_activity_code}</a>
                {:else}
                <a href={UI_USER_REPEATED_ACTIVITY.VIEW(reviewed_finished_activity.repeated_activity_code)}>{reviewed_finished_activity.repeated_activity_code}</a>
                {/if}
            <td>
        </tr>
{/each}
    </tbody>
</table>