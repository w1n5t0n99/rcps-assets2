
allow(user: User, action: String, resource: User) if
  has_permission(user, action, resource);

allow(user: User, "delete", resource: User) if
     has_permission(user, "delete", resource) and
     not user.id == resource.id;

allow(user: User, "update", resource: User) if
     has_permission(user, "update", resource) and
     not user.id == resource.id;

 
resource User {
    permissions = [
        # Update details about a User
        "update",
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
    "delete" if "admin";
    "create" if "admin";
    "view" if "admin";
}

actor User {}

has_role(user: User, role: String, resource: User) if
  user.role = role and
  user.organization_id = resource.organization_id;

