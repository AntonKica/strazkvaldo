import { SVC_ENUM_ACTIVITY_TYPE, SVC_ENUM_CRITICALITY_TYPE } from '$lib/serviceRoutes';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ }) => {
    let activity_type_result = await fetch(SVC_ENUM_ACTIVITY_TYPE.LIST()).then(response => {
        return response.json();
    });
    let criticality_type_result = await fetch(SVC_ENUM_CRITICALITY_TYPE.LIST()).then(response => {
        return response.json();
    });

    return {
        activity_types: activity_type_result.enum_values,
        criticality_types: criticality_type_result.enum_values,
    };
};
