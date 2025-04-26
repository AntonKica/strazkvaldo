import { SVC_USER_ONE_TIME_ACTIVITY } from '$lib/serviceRoutes';
import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
    let one_time_activities_result = await fetch(SVC_USER_ONE_TIME_ACTIVITY.LIST()).then(response => {
        return response.json()
    });
    return {
        ...one_time_activities_result,
    };
};