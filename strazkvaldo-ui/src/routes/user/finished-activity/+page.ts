import { SVC_USER_FINISHEd_ACTIVITY } from '$lib/serviceRoutes';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params }) => {
	const finished_activities_result = await fetch(SVC_USER_FINISHEd_ACTIVITY.LIST()).then(response => {
        return response.json()
    });
    return {
        finished_activities: finished_activities_result.finished_activities,
    };
};