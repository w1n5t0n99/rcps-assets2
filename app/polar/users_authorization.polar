
allow(user: Client, "create", resource: User) if
  has_permission(user, "create", resource);

allow(user: Client, "view", resource: User) if
  has_permission(user, "view", resource);

allow(user: Client, "update", resource: User) if
  has_permission(user, "update", resource);

allow(user: Client, "delete", resource: User) if
     has_permission(user, "delete", resource) and
     not user.user_id == resource.id;

allow(user: Client, "update_role", resource: User) if
     has_permission(user, "update_role", resource) and
     not user.user_id == resource.id;
 
resource User {
    permissions = [
        # Update details about a User
        "update",
        # Update a User's role
        "update_role",
        # Delete a User
        "delete",
        # Create a User
        "create",
        # View details about a User
        "view"
    ];
    roles = [
        "member",
        "manager",
        "admin"
    ];

    "update" if "admin";
    "update_role" if "admin";
    "delete" if "admin";
    "create" if "admin";
    "view" if "admin";
}

actor Client {}

has_role(user: Client, role: String, resource: User) if
  user.role == role and
  user.org_id == resource.organization_id;

