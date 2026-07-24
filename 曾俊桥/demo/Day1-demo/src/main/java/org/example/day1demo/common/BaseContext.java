package org.example.day1demo.common;

public class BaseContext {
    private static final ThreadLocal<Long> THREAD_LOCAL = new ThreadLocal<Long>();

    public static void setCurrentTenantId(Long tenantId) {
        THREAD_LOCAL.set(tenantId);
    }

    public static Long getCurrentTenantId() {
        return THREAD_LOCAL.get();
    }

    public static void removeCurrentTenantId() {
        THREAD_LOCAL.remove();
    }
}
