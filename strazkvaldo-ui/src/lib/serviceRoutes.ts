import { base } from '$app/paths';

class EntityRoute {
    roleName: string;
    entityName: string;

    constructor(roleName: string, entityName: string) {
        this.roleName = roleName;
        this.entityName = entityName;
    }

    basePath(): string {
        return `/svc/${this.roleName}/${this.entityName}`
    }
    LIST(): string {
        return this.basePath();
    }
    POST(): string {
        return this.basePath();
    }

    GET(code: string): string {
        return `${this.basePath()}/${code}`;
    }
    PATCH(code: string): string {
        return `${this.basePath()}/${code}`;
    }
};

export const SVC_ADMIN_APP_USER = new EntityRoute('admin', 'app-user');
export const SVC_USER_ONE_TIME_ACTIVITY = new EntityRoute('user', 'one-time-activity');