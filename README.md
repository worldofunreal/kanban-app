# Kanban App - Collaborative Project Management on Internet Computer

A modern, collaborative project and team management application built on the Internet Computer (ICP) with Rust backend and Vue.js frontend, inspired by Jira and Trello workflow builders.

## 🚀 Features

### Core Functionality
- **User Management**: Sign up, login, and profile management
- **Team Management**: Create, edit, and manage teams with privacy controls
- **Project Management**: Create projects owned by individuals or teams
- **Role-Based Access Control**: Owner, Manager, and Collaborator roles
- **Invitation System**: Invite users via links with role assignment
- **Modern UI**: Responsive design with Vue 3 and TailwindCSS

### User Roles
- **Owner**: Full permissions, can transfer ownership
- **Manager**: Can edit projects/teams and invite others
- **Collaborator**: Can use features but cannot manage teams/projects

### Privacy Features
- Public/private teams and projects
- Invite-only access control
- Role-based permissions

## 🛠 Tech Stack

### Backend (Internet Computer)
- **Language**: Rust
- **Framework**: DFX (Internet Computer SDK)
- **Authentication**: Internet Identity integration
- **Storage**: Canister stable memory
- **Interface**: Candid

### Frontend
- **Framework**: Vue 3 with Composition API
- **Build Tool**: Vite
- **Styling**: TailwindCSS
- **State Management**: Pinia
- **Routing**: Vue Router
- **UI Components**: Headless UI

## 📁 Project Structure

```
kanban-app/
├── src/
│   ├── backend/                 # Rust canister
│   │   ├── src/
│   │   │   ├── lib.rs          # Main canister logic
│   │   │   ├── models/         # Data structures
│   │   │   ├── handlers/       # Request handlers
│   │   │   └── utils/          # Utility functions
│   │   └── kanban-app-backend.did  # Candid interface
│   └── frontend/               # Vue.js application
│       ├── src/
│       │   ├── components/     # Vue components
│       │   ├── views/          # Page components
│       │   ├── stores/         # Pinia stores
│       │   ├── router/         # Vue Router configuration
│       │   └── utils/          # Utility functions
│       └── public/             # Static assets
├── dfx.json                    # DFX configuration
└── package.json               # Frontend dependencies
```

## 🚀 Getting Started

### Prerequisites
- [DFX](https://internetcomputer.org/docs/current/developer-docs/setup/install/) installed
- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://rustup.rs/) and Cargo

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd kanban-app
   ```

2. **Install frontend dependencies**
   ```bash
   cd src/frontend
   npm install
   ```

3. **Start local Internet Computer**
```bash
dfx start --background
   ```

4. **Deploy the canisters**
   ```bash
dfx deploy
```

5. **Start the frontend development server**
```bash
   cd src/frontend
   npm run dev
   ```

### Development Workflow

1. **Backend Development**
   - Edit Rust code in `src/backend/src/`
   - Update Candid interface in `src/backend/kanban-app-backend.did`
   - Deploy with `dfx deploy backend`

2. **Frontend Development**
   - Edit Vue components in `src/frontend/src/`
   - Run `npm run dev` for hot reload
   - Build with `npm run build`

## 📋 API Reference

### User Management
- `create_user(profile: UserProfile) -> Result<UserId, Error>`
- `get_user(user_id: UserId) -> Option<User>`
- `update_profile(user_id: UserId, profile: UserProfile) -> Result<(), Error>`

### Team Management
- `create_team(name: String, description: String, is_public: bool) -> Result<TeamId, Error>`
- `get_team(team_id: TeamId) -> Option<Team>`
- `update_team(team_id: TeamId, updates: TeamUpdate) -> Result<(), Error>`
- `delete_team(team_id: TeamId) -> Result<(), Error>`

### Project Management
- `create_project(name: String, description: String, owner: Owner) -> Result<ProjectId, Error>`
- `get_project(project_id: ProjectId) -> Option<Project>`
- `update_project(project_id: ProjectId, updates: ProjectUpdate) -> Result<(), Error>`
- `transfer_ownership(project_id: ProjectId, new_owner: Owner) -> Result<(), Error>`

### Access Control
- `invite_user(target: InviteTarget, role: Role, expires_at: Option<Timestamp>) -> Result<InviteId, Error>`
- `accept_invite(invite_id: InviteId) -> Result<(), Error>`
- `remove_member(target: InviteTarget, user_id: UserId) -> Result<(), Error>`

## 🔐 Security & Privacy

- **Internet Identity Integration**: Secure authentication via Internet Identity
- **Role-Based Access Control**: Granular permissions based on user roles
- **Invite-Only Access**: Private teams and projects require explicit invitations
- **Ownership Transfer**: Secure transfer of project/team ownership

## 🎨 UI/UX Features

- **Responsive Design**: Works on desktop, tablet, and mobile
- **Dark/Light Mode**: Toggle between themes
- **Real-time Updates**: Live collaboration features
- **Drag & Drop**: Intuitive project management interface
- **Search & Filter**: Easy navigation through projects and teams

## 🚧 Roadmap

### Phase 1 (Current)
- [x] Basic user authentication
- [x] Team and project creation
- [x] Role-based access control
- [x] Invitation system

### Phase 2
- [ ] Kanban board implementation
- [ ] Task management
- [ ] File attachments
- [ ] Activity feed

### Phase 3
- [ ] Advanced workflow automation
- [ ] Time tracking
- [ ] Reporting and analytics
- [ ] Mobile app

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Documentation**: Check the inline code comments and this README
- **Issues**: Report bugs and feature requests via GitHub Issues
- **Discussions**: Join the community discussions for questions and ideas

## 🔗 Links

- [Internet Computer Documentation](https://internetcomputer.org/docs/current/developer-docs/)
- [DFX Documentation](https://internetcomputer.org/docs/current/developer-docs/setup/install/)
- [Vue.js Documentation](https://vuejs.org/)
- [TailwindCSS Documentation](https://tailwindcss.com/)
