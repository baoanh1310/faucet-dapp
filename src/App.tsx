import React from 'react';
import { BrowserRouter as Router, Switch, Route } from 'react-router-dom';
import MainLayout from "~components/layouts/MainLayout";
import FaucetPage from "~pages/FaucetPage";

function App() {
  return (
    <Router>
        <MainLayout>
            <div className="relative pb-24 overflow-x-hidden xs:flex xs:flex-col md:flex md:flex-col">
                <Switch>
                    <Route path="/" component={AutoHeight(FaucetPage)} />
                </Switch>
            </div>
        </MainLayout>
    </Router>
  );
}

// decorate any components with this HOC to display them as vertical-align middle
// use individual fn is needed since `h-4/5` is not a appropriate style rule for
// any components
function AutoHeight(Comp: any) {
  return (props: any) => {
    return (
      <div className="xs:flex xs:flex-col md:flex md:flex-col justify-center h-4/5 lg:mt-12 relative xs:mt-8">
        <Comp {...props} />
      </div>
    );
  };
}

export default App;
