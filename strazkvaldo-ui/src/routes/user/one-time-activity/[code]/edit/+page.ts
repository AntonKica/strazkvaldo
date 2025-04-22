import { SVC_USER_ONE_TIME_ACTIVITY, SVC_ENUM_ACTIVITY_TYPE, SVC_ENUM_CRITICALITY_TYPE } from '$lib/serviceRoutes';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch }) => {
    let one_time_activity_result = await fetch(SVC_USER_ONE_TIME_ACTIVITY.GET(params.code)).then(response => {
        return response.json();
    });
    let activity_type_result = await fetch(SVC_ENUM_ACTIVITY_TYPE.LIST()).then(response => {
        return response.json();
    });
    let criticality_type_result = await fetch(SVC_ENUM_CRITICALITY_TYPE.LIST()).then(response => {
        return response.json();
    });

    return {
        one_time_activity: one_time_activity_result.data.one_time_activity,
        activity_types: activity_type_result.enum_values,
        criticality_types: criticality_type_result.enum_values,
    };
};