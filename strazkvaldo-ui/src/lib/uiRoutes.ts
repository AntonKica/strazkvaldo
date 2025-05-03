import { base } from '$app/paths';
import { goto } from '$app/navigation';

class EntityRouteUI {
    roleName: string;
    entityName: string;

    constructor(roleName: string, entityName: string) {
        this.roleName = roleName;
        this.entityName = entityName;
    }

    basePath(): string {
        return `${base}/${this.roleName}/${this.entityName}`
    }
    LIST(): string {
        return this.basePath();
    }
    VIEW(code: string): string {
        return `${this.basePath()}/${code}/view`;
    }
    EDIT(code: string): string {
        return `${this.basePath()}/${code}/edit`;
    }
    CREATE(): string {
        return `${this.basePath()}/create`;
    }
};

export const UI_ADMIN_APP_USER = new EntityRouteUI('admin', 'app-user');
export const UI_USER_ONE_TIME_ACTIVITY = new EntityRouteUI('user', 'one-time-activity');
export const UI_USER_REPEATED_ACTIVITY = new EntityRouteUI('user', 'repeated-activity');
export const UI_USER_RECENTLY_FINISHED_ACTIVITY = new EntityRouteUI('user', 'recently-finished-activity');
export const UI_USER_REVIEWED_FINISHED_ACTIVITY = new EntityRouteUI('user', 'reviewed-finished-activity');
export const UI_USER_ROOM = new EntityRouteUI('user', 'room');

export async function delete_entity(delete_url: string, goto_url: string) {
    if(!window.confirm("Naozaj chcete vymazať?")) {
        return;
    }
    
    const res = await fetch(delete_url, {
        method: "DELETE",
        headers: {
            'Content-Type': 'application/json' 
        },
    });

    if (!res.ok) {
        alert("Niečo sa pokazilo pri vymazávaní, skúste znovu.")
    } else {
        alert("Úspešné vymazaný.");
        goto(goto_url, { invalidateAll: true});
    }
}