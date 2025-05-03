<script lang="ts">
    import { base } from '$app/paths';
	import type { PageProps } from './$types';
    import { UI_USER_ONE_TIME_ACTIVITY, UI_USER_REPEATED_ACTIVITY } from '$lib/uiRoutes';

	let { data }: PageProps = $props();
    let upcomingActivities = data.upcoming_activities;
</script>

<h1>Nadchádzajúce aktivity</h1>

<table>
    <thead>
        <tr>
            <td>aktivita</td>
            <td>datum</td>
        </tr>
    </thead>
    <tbody>
{#each upcomingActivities as upcomingActivity}
		<tr>
            <td>
            {#if upcomingActivity.one_time_activity}
                <a href={UI_USER_ONE_TIME_ACTIVITY.VIEW(upcomingActivity.one_time_activity.code)}>{upcomingActivity.one_time_activity.name}</a>
            {:else}
                <a href={UI_USER_REPEATED_ACTIVITY.VIEW(upcomingActivity.repeated_activity.code)}>{upcomingActivity.repeated_activity.name}</a>
            {/if}
            </td>
            <td>{upcomingActivity.due_date}</td>
        </tr>
{/each}
    </tbody>
</table>